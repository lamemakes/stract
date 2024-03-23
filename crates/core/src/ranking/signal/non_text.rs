// Stract is an open source web search engine.
// Copyright (C) 2024 Stract ApS
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as
// published by the Free Software Foundation, either version 3 of the
// License, or (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>

use tantivy::DocId;

use super::{Signal, SignalComputer};
use crate::{
    fastfield_reader::FieldReader,
    schema::{self, Field, FLOAT_SCALING},
    webgraph::NodeID,
    webpage::Webpage,
};

fn score_timestamp(page_timestamp: usize, signal_computer: &SignalComputer) -> f64 {
    if page_timestamp >= signal_computer.current_timestamp().unwrap_or(0) {
        return 0.0;
    }

    let hours_since_update =
        (signal_computer.current_timestamp().unwrap() - page_timestamp).max(1) / 3600;

    signal_computer
        .update_time_cache()
        .get(hours_since_update)
        .copied()
        .unwrap_or(0.0)
}

#[inline]
fn score_rank(rank: f64) -> f64 {
    1.0 / (rank + 1.0)
}

#[inline]
fn score_trackers(num_trackers: f64) -> f64 {
    1.0 / (num_trackers + 1.0)
}

#[inline]
fn score_digits(num_digits: f64) -> f64 {
    1.0 / (num_digits + 1.0)
}

#[inline]
fn score_slashes(num_slashes: f64) -> f64 {
    1.0 / (num_slashes + 1.0)
}

#[inline]
fn score_link_density(link_density: f64) -> f64 {
    if link_density > 0.5 {
        0.0
    } else {
        1.0 - link_density
    }
}

fn score_region(webpage_region: crate::webpage::Region, computer: &SignalComputer) -> f64 {
    match computer.region_count() {
        Some(region_count) => {
            let boost = computer
                .query_data()
                .and_then(|q| q.selected_region())
                .map_or(0.0, |region| {
                    if region != crate::webpage::Region::All && region == webpage_region {
                        50.0
                    } else {
                        0.0
                    }
                });

            boost + region_count.score(&webpage_region)
        }
        None => 0.0,
    }
}

fn host_id(fastfield_reader: &FieldReader<'_>) -> Option<NodeID> {
    let node_id = fastfield_reader
        .get(schema::fast_field::HostNodeID.into())
        .and_then(|n| n.as_u64())
        .unwrap();

    if node_id == u64::MAX {
        None
    } else {
        Some(node_id.into())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct HostCentrality;
impl Signal for HostCentrality {
    fn default_coefficient(&self) -> f64 {
        0.5
    }

    fn as_field(&self) -> Option<Field> {
        Some(Field::Fast(schema::fast_field::HostCentrality.into()))
    }

    fn precompute(self, webpage: &Webpage, _: &SignalComputer) -> Option<f64> {
        Some(webpage.host_centrality)
    }

    fn compute(&self, doc: DocId, signal_computer: &SignalComputer) -> Option<f64> {
        let seg_reader = signal_computer.segment_reader().unwrap().borrow_mut();
        let fastfield_reader = seg_reader.fastfield_reader().get_field_reader(doc);

        let val = fastfield_reader
            .get(self.as_fastfield().unwrap())
            .and_then(|v| v.as_u64())
            .unwrap();
        Some(val as f64 / FLOAT_SCALING as f64)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct HostCentralityRank;
impl Signal for HostCentralityRank {
    fn default_coefficient(&self) -> f64 {
        0.0
    }

    fn as_field(&self) -> Option<Field> {
        Some(Field::Fast(schema::fast_field::HostCentralityRank.into()))
    }

    fn precompute(self, webpage: &Webpage, _: &SignalComputer) -> Option<f64> {
        Some(score_rank(webpage.host_centrality_rank as f64))
    }

    fn compute(&self, doc: DocId, signal_computer: &SignalComputer) -> Option<f64> {
        let seg_reader = signal_computer.segment_reader().unwrap().borrow_mut();
        let fastfield_reader = seg_reader.fastfield_reader().get_field_reader(doc);

        let val = fastfield_reader
            .get(self.as_fastfield().unwrap())
            .and_then(|v| v.as_u64())
            .unwrap();
        Some(score_rank(val as f64))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct PageCentrality;
impl Signal for PageCentrality {
    fn default_coefficient(&self) -> f64 {
        0.25
    }

    fn as_field(&self) -> Option<Field> {
        Some(Field::Fast(schema::fast_field::PageCentrality.into()))
    }

    fn precompute(self, webpage: &Webpage, _: &SignalComputer) -> Option<f64> {
        Some(webpage.page_centrality)
    }

    fn compute(&self, doc: DocId, signal_computer: &SignalComputer) -> Option<f64> {
        let seg_reader = signal_computer.segment_reader().unwrap().borrow_mut();
        let fastfield_reader = seg_reader.fastfield_reader().get_field_reader(doc);

        let val = fastfield_reader
            .get(self.as_fastfield().unwrap())
            .and_then(|v| v.as_u64())
            .unwrap();
        Some(val as f64 / FLOAT_SCALING as f64)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct PageCentralityRank;
impl Signal for PageCentralityRank {
    fn default_coefficient(&self) -> f64 {
        0.0
    }

    fn as_field(&self) -> Option<Field> {
        Some(Field::Fast(schema::fast_field::PageCentralityRank.into()))
    }

    fn precompute(self, webpage: &Webpage, _: &SignalComputer) -> Option<f64> {
        Some(score_rank(webpage.page_centrality_rank as f64))
    }

    fn compute(&self, doc: DocId, signal_computer: &SignalComputer) -> Option<f64> {
        let seg_reader = signal_computer.segment_reader().unwrap().borrow_mut();
        let fastfield_reader = seg_reader.fastfield_reader().get_field_reader(doc);

        let val = fastfield_reader
            .get(self.as_fastfield().unwrap())
            .and_then(|v| v.as_u64())
            .unwrap();
        Some(score_rank(val as f64))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct IsHomepage;
impl Signal for IsHomepage {
    fn default_coefficient(&self) -> f64 {
        0.0005
    }

    fn as_field(&self) -> Option<Field> {
        Some(Field::Fast(schema::fast_field::IsHomepage.into()))
    }

    fn precompute(self, webpage: &Webpage, _: &SignalComputer) -> Option<f64> {
        Some(webpage.html.is_homepage().into())
    }

    fn compute(&self, doc: DocId, signal_computer: &SignalComputer) -> Option<f64> {
        let seg_reader = signal_computer.segment_reader().unwrap().borrow_mut();
        let fastfield_reader = seg_reader.fastfield_reader().get_field_reader(doc);

        let val = fastfield_reader
            .get(self.as_fastfield().unwrap())
            .and_then(|v| v.as_u64())
            .unwrap();
        Some(val as f64)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct FetchTimeMs;
impl Signal for FetchTimeMs {
    fn default_coefficient(&self) -> f64 {
        0.001
    }

    fn as_field(&self) -> Option<Field> {
        Some(Field::Fast(schema::fast_field::FetchTimeMs.into()))
    }

    fn precompute(self, webpage: &Webpage, signal_computer: &SignalComputer) -> Option<f64> {
        let fetch_time_ms = webpage.fetch_time_ms as usize;
        if fetch_time_ms >= signal_computer.fetch_time_ms_cache().len() {
            Some(0.0)
        } else {
            Some(signal_computer.fetch_time_ms_cache()[fetch_time_ms])
        }
    }

    fn compute(&self, doc: DocId, signal_computer: &SignalComputer) -> Option<f64> {
        let seg_reader = signal_computer.segment_reader().unwrap().borrow_mut();
        let fastfield_reader = seg_reader.fastfield_reader().get_field_reader(doc);

        let fetch_time_ms = fastfield_reader
            .get(self.as_fastfield().unwrap())
            .and_then(|v| v.as_u64())
            .unwrap() as usize;

        if fetch_time_ms >= signal_computer.fetch_time_ms_cache().len() {
            Some(0.0)
        } else {
            Some(signal_computer.fetch_time_ms_cache()[fetch_time_ms])
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct UpdateTimestamp;
impl Signal for UpdateTimestamp {
    fn default_coefficient(&self) -> f64 {
        0.001
    }

    fn as_field(&self) -> Option<Field> {
        Some(Field::Fast(schema::fast_field::LastUpdated.into()))
    }

    fn precompute(self, webpage: &Webpage, signal_computer: &SignalComputer) -> Option<f64> {
        let update_timestamp = webpage
            .html
            .updated_time()
            .map(|date| date.timestamp().max(0))
            .unwrap_or(0) as usize;

        Some(score_timestamp(update_timestamp, signal_computer))
    }

    fn compute(&self, doc: DocId, signal_computer: &SignalComputer) -> Option<f64> {
        let seg_reader = signal_computer.segment_reader().unwrap().borrow_mut();
        let fastfield_reader = seg_reader.fastfield_reader().get_field_reader(doc);

        let val = fastfield_reader
            .get(self.as_fastfield().unwrap())
            .and_then(|v| v.as_u64())
            .unwrap() as usize;

        Some(score_timestamp(val, signal_computer))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct TrackerScore;
impl Signal for TrackerScore {
    fn default_coefficient(&self) -> f64 {
        0.05
    }

    fn as_field(&self) -> Option<Field> {
        Some(Field::Fast(schema::fast_field::TrackerScore.into()))
    }

    fn precompute(self, webpage: &Webpage, _: &SignalComputer) -> Option<f64> {
        let num_trackers = webpage.html.trackers().len() as f64;
        Some(score_trackers(num_trackers))
    }

    fn compute(&self, doc: DocId, signal_computer: &SignalComputer) -> Option<f64> {
        let seg_reader = signal_computer.segment_reader().unwrap().borrow_mut();
        let fastfield_reader = seg_reader.fastfield_reader().get_field_reader(doc);

        let val = fastfield_reader
            .get(self.as_fastfield().unwrap())
            .and_then(|v| v.as_u64())
            .unwrap();
        Some(score_trackers(val as f64))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct Region;
impl Signal for Region {
    fn default_coefficient(&self) -> f64 {
        0.15
    }

    fn as_field(&self) -> Option<Field> {
        Some(Field::Fast(schema::fast_field::Region.into()))
    }

    fn precompute(self, webpage: &Webpage, signal_computer: &SignalComputer) -> Option<f64> {
        let region =
            crate::webpage::Region::guess_from(webpage).unwrap_or(crate::webpage::Region::All);
        Some(score_region(region, signal_computer))
    }

    fn compute(&self, doc: DocId, signal_computer: &SignalComputer) -> Option<f64> {
        let seg_reader = signal_computer.segment_reader().unwrap().borrow_mut();
        let fastfield_reader = seg_reader.fastfield_reader().get_field_reader(doc);

        let val = fastfield_reader
            .get(self.as_fastfield().unwrap())
            .and_then(|v| v.as_u64())
            .unwrap();
        let region = crate::webpage::Region::from_id(val);
        Some(score_region(region, signal_computer))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct QueryCentrality;
impl Signal for QueryCentrality {
    fn default_coefficient(&self) -> f64 {
        0.0
    }

    fn as_field(&self) -> Option<Field> {
        None
    }

    fn compute(&self, doc: DocId, signal_computer: &SignalComputer) -> Option<f64> {
        let seg_reader = signal_computer.segment_reader().unwrap().borrow_mut();
        let fastfield_reader = seg_reader.fastfield_reader().get_field_reader(doc);
        let host_id = host_id(&fastfield_reader);

        host_id.and_then(|host_id| signal_computer.query_centrality(host_id))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct InboundSimilarity;
impl Signal for InboundSimilarity {
    fn default_coefficient(&self) -> f64 {
        0.25
    }

    fn as_field(&self) -> Option<Field> {
        None
    }

    fn compute(&self, doc: DocId, signal_computer: &SignalComputer) -> Option<f64> {
        let seg_reader = signal_computer.segment_reader().unwrap().borrow_mut();
        let fastfield_reader = seg_reader.fastfield_reader().get_field_reader(doc);
        let host_id = host_id(&fastfield_reader);

        host_id.map(|host_id| signal_computer.inbound_similarity(host_id))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct LambdaMart;
impl Signal for LambdaMart {
    fn default_coefficient(&self) -> f64 {
        10.0
    }

    fn as_field(&self) -> Option<Field> {
        None
    }

    fn compute(&self, _: DocId, _: &SignalComputer) -> Option<f64> {
        None // computed in later ranking stage
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct UrlDigits;
impl Signal for UrlDigits {
    fn default_coefficient(&self) -> f64 {
        0.01
    }

    fn as_field(&self) -> Option<Field> {
        Some(Field::Fast(
            schema::fast_field::NumPathAndQueryDigits.into(),
        ))
    }

    fn precompute(self, webpage: &Webpage, _: &SignalComputer) -> Option<f64> {
        let num_digits = (webpage
            .html
            .url()
            .path()
            .chars()
            .filter(|c| c.is_ascii_digit())
            .count()
            + webpage
                .html
                .url()
                .query()
                .unwrap_or_default()
                .chars()
                .filter(|c| c.is_ascii_digit())
                .count()) as f64;

        Some(score_digits(num_digits))
    }

    fn compute(&self, doc: DocId, signal_computer: &SignalComputer) -> Option<f64> {
        let seg_reader = signal_computer.segment_reader().unwrap().borrow_mut();
        let fastfield_reader = seg_reader.fastfield_reader().get_field_reader(doc);

        let val = fastfield_reader
            .get(self.as_fastfield().unwrap())
            .and_then(|v| v.as_u64())
            .unwrap();
        Some(score_digits(val as f64))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct UrlSlashes;
impl Signal for UrlSlashes {
    fn default_coefficient(&self) -> f64 {
        0.01
    }

    fn as_field(&self) -> Option<Field> {
        Some(Field::Fast(
            schema::fast_field::NumPathAndQuerySlashes.into(),
        ))
    }

    fn precompute(self, webpage: &Webpage, _: &SignalComputer) -> Option<f64> {
        let num_slashes = webpage
            .html
            .url()
            .path()
            .chars()
            .filter(|c| c == &'/')
            .count() as f64;
        Some(score_slashes(num_slashes))
    }

    fn compute(&self, doc: DocId, signal_computer: &SignalComputer) -> Option<f64> {
        let seg_reader = signal_computer.segment_reader().unwrap().borrow_mut();
        let fastfield_reader = seg_reader.fastfield_reader().get_field_reader(doc);

        let val = fastfield_reader
            .get(self.as_fastfield().unwrap())
            .and_then(|v| v.as_u64())
            .unwrap();
        Some(score_slashes(val as f64))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct LinkDensity;
impl Signal for LinkDensity {
    fn default_coefficient(&self) -> f64 {
        0.0
    }

    fn as_field(&self) -> Option<Field> {
        Some(Field::Fast(schema::fast_field::LinkDensity.into()))
    }

    fn precompute(self, webpage: &Webpage, _: &SignalComputer) -> Option<f64> {
        let link_density = webpage.html.link_density();
        Some(score_link_density(link_density))
    }

    fn compute(&self, doc: DocId, signal_computer: &SignalComputer) -> Option<f64> {
        let seg_reader = signal_computer.segment_reader().unwrap().borrow_mut();
        let fastfield_reader = seg_reader.fastfield_reader().get_field_reader(doc);

        let val = fastfield_reader
            .get(self.as_fastfield().unwrap())
            .and_then(|v| v.as_u64())
            .unwrap();
        Some(score_link_density(val as f64 / FLOAT_SCALING as f64))
    }
}
