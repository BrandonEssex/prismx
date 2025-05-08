#[derive(Debug, Clone)]
pub struct SearchResult {
    pub title: String,
    pub score: i64,
}

pub struct SpotlightEngine;

impl SpotlightEngine {
    pub fn new() -> Self {
        SpotlightEngine
    }

    pub fn search(&self, query: &str) -> Vec<SearchResult> {
        if query.is_empty() {
            vec![]
        } else {
            vec![
                SearchResult { title: format!("Match for: {}", query), score: 100 },
                SearchResult { title: "Example result".into(), score: 80 },
            ]
        }
    }
}