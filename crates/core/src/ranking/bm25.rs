// source: https://github.com/quickwit-oss/tantivy/blob/main/src/query/bm25.rs

use itertools::Itertools;

use tantivy::fieldnorm::FieldNormReader;
use tantivy::query::Explanation;
use tantivy::{Score, Searcher, Term};

const K1: Score = 1.2;
const B: Score = 0.75;

pub(crate) fn idf(doc_freq: u64, doc_count: u64) -> Score {
    assert!(doc_count >= doc_freq, "{doc_count} >= {doc_freq}");
    let x = ((doc_count - doc_freq) as Score + 0.5) / (doc_freq as Score + 0.5);
    (1.0 + x).ln()
}

fn cached_tf_component(fieldnorm: u32, average_fieldnorm: Score) -> Score {
    K1 * (1.0 - B + B * fieldnorm as Score / average_fieldnorm)
}

fn compute_tf_cache(average_fieldnorm: Score) -> [Score; 256] {
    let mut cache: [Score; 256] = [0.0; 256];
    for (fieldnorm_id, cache_mut) in cache.iter_mut().enumerate() {
        let fieldnorm = FieldNormReader::id_to_fieldnorm(fieldnorm_id as u8);
        *cache_mut = cached_tf_component(fieldnorm, average_fieldnorm);
    }
    cache
}

#[derive(
    Clone, PartialEq, Debug, serde::Serialize, serde::Deserialize, bincode::Encode, bincode::Decode,
)]
pub struct Bm25Params {
    pub idf: Score,
    pub avg_fieldnorm: Score,
}

#[derive(Clone)]
pub struct MultiBm25Weight {
    weights: Vec<Bm25Weight>,
}

impl MultiBm25Weight {
    pub fn for_terms(searcher: &Searcher, terms: &[Term]) -> tantivy::Result<Self> {
        if terms.is_empty() {
            return Ok(Self {
                weights: Vec::new(),
            });
        }

        let field = terms[0].field();
        for term in terms.iter().skip(1) {
            assert_eq!(
                term.field(),
                field,
                "All terms must belong to the same field."
            );
        }

        let mut total_num_tokens = 0u64;
        let mut total_num_docs = 0u64;
        for segment_reader in searcher.segment_readers() {
            let inverted_index = segment_reader.inverted_index(field)?;
            total_num_tokens += inverted_index.total_num_tokens();
            total_num_docs += u64::from(segment_reader.max_doc());
        }
        let average_fieldnorm = total_num_tokens as Score / total_num_docs as Score;

        let mut weights = Vec::new();

        for term in terms {
            let term_doc_freq = searcher.doc_freq(term)?;
            weights.push(Bm25Weight::for_one_term(
                term_doc_freq,
                total_num_docs,
                average_fieldnorm,
            ));
        }

        Ok(Self { weights })
    }

    #[inline]
    pub fn score(&self, stats: impl Iterator<Item = (u8, u32)>) -> Score {
        stats
            .zip_eq(self.weights.iter())
            .map(|((fieldnorm_id, term_freq), weight)| weight.score(fieldnorm_id, term_freq))
            .sum()
    }

    pub fn idf(&self) -> impl Iterator<Item = f32> + '_ {
        self.weights.iter().map(|w| w.weight)
    }

    pub fn boost_by(&self, boost: Score) -> Self {
        Self {
            weights: self.weights.iter().map(|w| w.boost_by(boost)).collect(),
        }
    }
}

#[derive(Clone)]
pub struct Bm25Weight {
    idf_explain: Explanation,
    weight: Score,
    cache: [Score; 256],
    average_fieldnorm: Score,
}

impl Bm25Weight {
    pub fn boost_by(&self, boost: Score) -> Bm25Weight {
        Bm25Weight {
            idf_explain: self.idf_explain.clone(),
            weight: self.weight * boost,
            cache: self.cache,
            average_fieldnorm: self.average_fieldnorm,
        }
    }

    pub fn for_one_term(
        term_doc_freq: u64,
        total_num_docs: u64,
        avg_fieldnorm: Score,
    ) -> Bm25Weight {
        let idf = idf(term_doc_freq, total_num_docs);
        let mut idf_explain =
            Explanation::new("idf, computed as log(1 + (N - n + 0.5) / (n + 0.5))", idf);
        idf_explain.add_const(
            "n, number of docs containing this term",
            term_doc_freq as Score,
        );
        idf_explain.add_const("N, total number of docs", total_num_docs as Score);
        Bm25Weight::new(idf_explain, avg_fieldnorm)
    }

    pub(crate) fn new(idf_explain: Explanation, average_fieldnorm: Score) -> Bm25Weight {
        let weight = idf_explain.value() * (1.0 + K1);
        Bm25Weight {
            idf_explain,
            weight,
            cache: compute_tf_cache(average_fieldnorm),
            average_fieldnorm,
        }
    }

    #[inline]
    pub fn score(&self, fieldnorm_id: u8, term_freq: u32) -> Score {
        self.weight * self.tf_factor(fieldnorm_id, term_freq)
    }

    #[inline]
    pub(crate) fn tf_factor(&self, fieldnorm_id: u8, term_freq: u32) -> Score {
        let term_freq = term_freq as Score;
        let norm = self.cache[fieldnorm_id as usize];
        term_freq / (term_freq + norm)
    }

    pub fn explain(&self, fieldnorm_id: u8, term_freq: u32) -> Explanation {
        // The explain format is directly copied from Lucene's.
        // (So, Kudos to Lucene)
        let score = self.score(fieldnorm_id, term_freq);

        let norm = self.cache[fieldnorm_id as usize];
        let term_freq = term_freq as Score;
        let right_factor = term_freq / (term_freq + norm);

        let mut tf_explanation = Explanation::new(
            "freq / (freq + k1 * (1 - b + b * dl / avgdl))",
            right_factor,
        );

        tf_explanation.add_const("freq, occurrences of term within document", term_freq);
        tf_explanation.add_const("k1, term saturation parameter", K1);
        tf_explanation.add_const("b, length normalization parameter", B);
        tf_explanation.add_const(
            "dl, length of field",
            FieldNormReader::id_to_fieldnorm(fieldnorm_id) as Score,
        );
        tf_explanation.add_const("avgdl, average length of field", self.average_fieldnorm);

        let mut explanation = Explanation::new("TermQuery, product of...", score);
        explanation.add_detail(Explanation::new("(K1+1)", K1 + 1.0));
        explanation.add_detail(self.idf_explain.clone());
        explanation.add_detail(tf_explanation);
        explanation
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bm25_idf_scaling() {
        // assume the query is something like 'the end'
        // 'the' appears in almost all docs (98)
        // 'end' appears in a smalle subset (20)
        let weight = MultiBm25Weight {
            weights: vec![
                Bm25Weight::for_one_term(98, 100, 1.0),
                Bm25Weight::for_one_term(20, 100, 1.0),
            ],
        };

        // if a document has high frequency of 'end'
        // it should have a higher score than a document that
        // has an almost equally high frequency of 'the'
        let high_the = weight.score(vec![(0, 15), (0, 10)].into_iter());
        let high_end = weight.score(vec![(0, 8), (0, 13)].into_iter());
        assert!(high_end > high_the);
    }
}
