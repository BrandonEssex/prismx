use crate::spotlight::engine::{SpotlightEngine, SearchResult};

#[derive(Debug)]
pub struct SpotlightState {
    pub engine: SpotlightEngine,
    pub matched: Vec<SearchResult>,
    pub query: String,
}