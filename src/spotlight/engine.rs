use crate::spotlight::plugin::{SearchScope, Searchable};
use fuzzy_matcher::skim::SkimMatcherV2;
use fuzzy_matcher::FuzzyMatcher;
use std::sync::Arc;

#[derive(Debug)]
pub struct SearchResult {
    pub title: String,
}

#[derive(Debug)]
pub struct SpotlightEngine {
    matcher: SkimMatcherV2,
    items: Vec<Arc<dyn Searchable>>,
}

impl SpotlightEngine {
    pub fn new() -> Self {
        SpotlightEngine {
            matcher: SkimMatcherV2::default(),
            items: vec![],
        }
    }

    pub fn update_items(&mut self, items: Vec<Arc<dyn Searchable>>) {
        self.items = items;
    }

    pub fn search(&self, query: &str) -> Vec<SearchResult> {
        let mut results = vec![];

        for item in &self.items {
            if let Some((_, _)) = self.matcher.fuzzy_match(&item.searchable_text(), query) {
                results.push(SearchResult {
                    title: item.display_title(),
                });
            }
        }

        results
    }
}