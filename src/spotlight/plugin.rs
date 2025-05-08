use std::sync::Arc;

pub trait Searchable: Send + Sync {
    fn searchable_text(&self) -> String;
    fn display_title(&self) -> String;
}

pub trait SearchableSource: Send + Sync {
    fn collect_items(&self) -> Vec<Arc<dyn Searchable>>;
}