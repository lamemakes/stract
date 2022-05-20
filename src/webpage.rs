use itertools::Itertools;
use scraper::{Html, Selector};
use std::collections::BTreeMap;

#[derive(Debug)]
pub(crate) struct Webpage {
    dom: Html,
}

impl Webpage {
    pub(crate) fn parse(html: &str) -> Self {
        Self {
            dom: Html::parse_document(html),
        }
    }

    pub(crate) fn links(&self) -> Vec<Link> {
        let selector = Selector::parse("a").expect("Failed to parse selector");
        self.dom
            .select(&selector)
            .map(|el| {
                let destination = el.value().attr("href");
                let text = el.text().collect::<String>();

                (destination, text)
            })
            .filter(|(dest, _)| dest.is_some())
            .map(|(destination, text)| {
                let destination = destination.unwrap().to_string();
                Link { destination, text }
            })
            .collect()
    }

    fn grab_texts(&self, selector: &Selector) -> Vec<String> {
        self.dom
            .select(selector)
            .flat_map(|el| el.text())
            .map(|t| String::from(t.trim()))
            .filter(|t| !t.is_empty())
            .collect::<Vec<String>>()
    }

    pub(crate) fn text(&self) -> String {
        let selector = Selector::parse("body").expect("Failed to parse selector");
        Itertools::intersperse(self.grab_texts(&selector).into_iter(), "\n".to_string())
            .collect::<String>()
            .trim()
            .to_string()
    }

    pub(crate) fn title(&self) -> Option<String> {
        let selector = Selector::parse("title").expect("Failed to parse selector");
        self.grab_texts(&selector).get(0).cloned()
    }

    pub(crate) fn metadata(&self) -> Vec<Meta> {
        let selector = Selector::parse("meta").expect("Failed to parse selector");
        self.dom
            .select(&selector)
            .map(|el| {
                el.value()
                    .attrs()
                    .into_iter()
                    .map(|(k, v)| (k.to_string(), v.to_string()))
                    .collect::<BTreeMap<String, String>>()
            })
            .collect()
    }
}

#[derive(Debug, PartialEq)]
pub(crate) struct Link {
    destination: String,
    text: String,
}

pub(crate) type Meta = BTreeMap<String, String>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple() {
        let raw = r#"
            <html>
                <head>
                    <title>Best website</title>
                    <meta name="meta1" content="value">
                </head>
                <body>
                    <a href="example.com">Best example website ever</a>
                </body>
            </html>
        "#;

        let webpage = Webpage::parse(raw);

        assert_eq!(&webpage.text(), "Best example website ever");
        assert_eq!(webpage.title(), Some("Best website".to_string()));
        assert_eq!(
            webpage.links(),
            vec![Link {
                destination: "example.com".to_string(),
                text: "Best example website ever".to_string()
            }]
        );

        let mut expected_meta = BTreeMap::new();
        expected_meta.insert("name".to_string(), "meta1".to_string());
        expected_meta.insert("content".to_string(), "value".to_string());

        assert_eq!(webpage.metadata(), vec![expected_meta]);
    }
}
