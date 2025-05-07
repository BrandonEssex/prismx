use std::sync::Arc;
use fuzzy_matcher::skim::SkimMatcherV2;
use fuzzy_matcher::FuzzyMatcher;
use super::plugin::{SearchScope, Searchable};

#[derive(Clone)]
pub struct SearchResult {
    pub uid: String,
    pub display_title: String,
    pub score: i64,
}

#[derive(Default)]
pub struct SpotlightEngine {
    matcher: SkimMatcherV2,
    sources: Vec<Arc<dyn Searchable>>,
}

impl SpotlightEngine {
    pub fn search(&self, query: &str, _scope: SearchScope) -> Vec<SearchResult> {
        let mut results = Vec::new();

        for item in &self.sources {
            if let Some(score) = self.matcher.fuzzy_match(&item.searchable_text(), query) {
                results.push(SearchResult {
                    uid: item.uid(),
                    display_title: item.display_title(),
                    score,
                });
            }
        }

        results.sort_by(|a, b| b.score.cmp(&a.score));
        results
    }

    pub fn update_sources(&mut self, sources: Vec<Arc<dyn Searchable>>) {
        self.sources = sources;
    }
}