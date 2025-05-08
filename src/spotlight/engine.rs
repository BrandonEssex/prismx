// src/spotlight/engine.rs

#[derive(Debug)]
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
            vec![]
        } else {
            vec![
                SearchResult { title: format!("Result for '{}'", query) },
                SearchResult { title: "Another result".into() },
            ]
        }
    }
}