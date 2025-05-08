#[derive(Debug, Clone)]
pub struct SearchResult {
    pub title: String,
    pub score: i64,
}

#[derive(Debug)]
pub struct SpotlightEngine;

impl SpotlightEngine {
    pub fn new() -> Self {
        Self
    }

    pub fn search(&self, query: &str) -> Vec<SearchResult> {
        if query.is_empty() {
            return vec![];
        }

        // Placeholder search logic — returns mock result
        vec![SearchResult {
            title: format!("Result for '{}'", query),
            score: 100,
        }]
    }
}