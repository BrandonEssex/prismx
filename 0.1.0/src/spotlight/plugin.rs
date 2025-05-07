use std::collections::HashMap;
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

pub struct PluginRegistry {
    sources: Vec<Arc<dyn Searchable>>,
}

impl PluginRegistry {
    pub fn new() -> Self {
        PluginRegistry { sources: Vec::new() }
    }

    pub fn register(&mut self, source: Arc<dyn Searchable>) {
        self.sources.push(source);
    }

    pub fn collect_items(&self) -> Vec<Arc<dyn Searchable>> {
        self.sources.clone()
    }
}