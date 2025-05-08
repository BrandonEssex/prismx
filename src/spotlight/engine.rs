#[derive(Debug, Clone)]
pub struct SearchResult {
    pub title: String,
}

#[derive(Debug)]
pub struct SpotlightEngine;

impl SpotlightEngine {
    pub fn new() -> Self {
        SpotlightEngine
    }

    pub fn search(&self, query: &str) -> Vec<SearchResult> {
        if query.is_empty() {
            return vec![];
        }

        vec![
            SearchResult {
                title: format!("Result for: {}", query),
            },
            SearchResult {
                title: "Second result".into(),
            },
        ]
    }
}