use std::sync::Arc;

pub enum SearchScope {
    All,
}

pub trait Searchable: Send + Sync {
    fn title(&self) -> String;
}

pub trait SearchableSource: Send + Sync {
    fn items(&self) -> Vec<Arc<dyn Searchable>>;
}