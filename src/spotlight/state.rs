use super::engine::{SearchResult, SpotlightEngine};

#[derive(Debug)]
pub struct SpotlightState {
    pub query: String,
    pub matched: Vec<SearchResult>,
    pub selected: usize,
    pub debug_enabled: bool,
    pub engine: SpotlightEngine,
}

impl SpotlightState {
    pub fn new() -> Self {
        Self {
            query: String::new(),
            matched: Vec::new(),
            selected: 0,
            debug_enabled: false,
            engine: SpotlightEngine::default(),
        }
    }

    pub fn refresh_matches(&mut self) {
        self.matched = self.engine.search(&self.query);
        self.selected = 0;
    }
}