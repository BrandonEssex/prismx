use crate::spotlight::plugin::{Searchable};
use skim::prelude::*;
use std::sync::Arc;

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

    pub fn search(&self, query: &str) -> Vec<crate::spotlight::engine::SearchResult> {
        self.items.iter()
            .filter_map(|item| {
                self.matcher.fuzzy_match(&item.searchable_text(), query)
                    .map(|score| SearchResult {
                        title: item.display_title(),
                        score,
                    })
            })
            .collect()
    }
}

pub struct SearchResult {
    pub title: String,
    pub score: i64,
}