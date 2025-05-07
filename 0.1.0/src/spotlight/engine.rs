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

pub struct SpotlightEngine {
    matcher: SkimMatcherV2,
    notes: Vec<Arc<dyn Searchable>>,
    todos: Vec<Arc<dyn Searchable>>,
    projects: Vec<Arc<dyn Searchable>>,
    plugins: Vec<Arc<dyn Searchable>>,
}

impl SpotlightEngine {
    pub fn new() -> Self {
        SpotlightEngine {
            matcher: SkimMatcherV2::default(),
            notes: vec![],
            todos: vec![],
            projects: vec![],
            plugins: vec![],
        }
    }

    pub fn update_notes(&mut self, items: Vec<Arc<dyn Searchable>>) {
        self.notes = items;
    }

    pub fn update_todos(&mut self, items: Vec<Arc<dyn Searchable>>) {
        self.todos = items;
    }

    pub fn update_projects(&mut self, items: Vec<Arc<dyn Searchable>>) {
        self.projects = items;
    }

    pub fn update_plugins(&mut self, items: Vec<Arc<dyn Searchable>>) {
        self.plugins = items;
    }

    pub fn search(&self, query: &str, scope: SearchScope) -> Vec<SearchResult> {
        let items = match scope {
            SearchScope::Notes => &self.notes,
            SearchScope::Todos => &self.todos,
            SearchScope::Projects => &self.projects,
            SearchScope::Plugins => &self.plugins,
            SearchScope::All => {
                let mut all = Vec::new();
                all.extend(self.notes.iter().cloned());
                all.extend(self.todos.iter().cloned());
                all.extend(self.projects.iter().cloned());
                all.extend(self.plugins.iter().cloned());
                return self.run_search(query, all);
            }
        };

        self.run_search(query, items.clone())
    }

    fn run_search(&self, query: &str, items: Vec<Arc<dyn Searchable>>) -> Vec<SearchResult> {
        let mut results = Vec::new();

        for item in items {
            if let Some((score, _indices)) = self.matcher.fuzzy_match(&item.searchable_text(), query).map(|s| (s, vec![])) {
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
}