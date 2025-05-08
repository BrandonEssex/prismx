// src/spotlight/plugin.rs

use std::sync::Arc;

#[derive(Clone, Debug)]
pub enum SearchScope {
    All,
    Notes,
    Todos,
    Projects,
    Plugins,
}

pub trait Searchable: Send + Sync {
    fn uid(&self) -> String;
    fn searchable_text(&self) -> String;
    fn display_title(&self) -> String;
}

pub trait SearchableSource: Send + Sync {
    fn items(&self) -> Vec<Arc<dyn Searchable>>;
}

pub struct PluginRegistry {
    sources: Vec<Arc<dyn SearchableSource>>,
}

impl PluginRegistry {
    pub fn new() -> Self {
        Self {
            sources: Vec::new(),
        }
    }

    pub fn register(&mut self, source: Arc<dyn SearchableSource>) {
        self.sources.push(source);
    }

    pub fn collect_items(&self) -> Vec<Arc<dyn Searchable>> {
        self.sources.iter().flat_map(|s| s.items()).collect()
    }
}