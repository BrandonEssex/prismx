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
    fn category(&self) -> SearchScope;
    fn metadata(&self) -> Option<HashMap<String, String>>;
}

pub trait SearchableSource: Send + Sync {
    fn name(&self) -> String;
    fn items(&self) -> Vec<Arc<dyn Searchable>>;
}

pub struct PluginRegistry {
    sources: Vec<Arc<dyn SearchableSource>>,
}

impl PluginRegistry {
    pub fn new() -> Self {
        PluginRegistry {
            sources: Vec::new(),
        }
    }

    pub fn register(&mut self, source: Arc<dyn SearchableSource>) {
        self.sources.push(source);
    }

    pub fn collect_items(&self) -> Vec<Arc<dyn Searchable>> {
        let mut all = vec![];
        for source in &self.sources {
            all.extend(source.items());
        }
        all
    }

    pub fn list_sources(&self) -> Vec<String> {
        self.sources.iter().map(|s| s.name()).collect()
    }
}