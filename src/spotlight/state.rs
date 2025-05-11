use crate::spotlight::engine::{SpotlightEngine, SearchResult};

pub struct SpotlightState {
    pub engine: SpotlightEngine,
    pub matched: Vec<SearchResult>,
    pub query: String,
}