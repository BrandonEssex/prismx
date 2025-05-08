#[derive(Clone, Debug)]
pub enum SearchScope {
    All,
}

pub trait Searchable: Send + Sync {
    fn uid(&self) -> String;
    fn searchable_text(&self) -> String;
    fn display_title(&self) -> String;
}

pub trait SearchableSource: Send + Sync {
    fn name(&self) -> String;
    fn items(&self) -> Vec<Box<dyn Searchable>>;
}

pub struct PluginRegistry {
    sources: Vec<Box<dyn SearchableSource>>,
}

impl PluginRegistry {
    pub fn new() -> Self {
        Self {
            sources: vec![],
        }
    }

    pub fn register(&mut self, source: Box<dyn SearchableSource>) {
        self.sources.push(source);
    }

    pub fn collect_items(&self) -> Vec<Box<dyn Searchable>> {
        self.sources.iter().flat_map(|s| s.items()).collect()
    }
}