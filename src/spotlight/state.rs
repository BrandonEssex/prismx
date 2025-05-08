use crate::spotlight::engine::{SearchResult, SpotlightEngine};
use crate::spotlight::plugin::SearchScope;

#[derive(Debug)]
pub struct SpotlightState {
    pub query: String,
    pub matched: Vec<SearchResult>,
    pub selected: usize,
    pub debug_enabled: bool,
    engine: SpotlightEngine,
}

impl SpotlightState {
    pub fn new() -> Self {
        SpotlightState {
            query: String::new(),
            matched: vec![],
            selected: 0,
            debug_enabled: false,
            engine: SpotlightEngine::new(),
        }
    }

    pub fn open(&mut self) {
        self.query.clear();
        self.refresh_matches();
    }

    pub fn update_query(&mut self, c: char) {
        self.query.push(c);
        self.refresh_matches();
    }

    pub fn backspace(&mut self) {
        self.query.pop();
        self.refresh_matches();
    }

    pub fn move_up(&mut self) {
        if self.selected > 0 {
            self.selected -= 1;
        }
    }

    pub fn move_down(&mut self) {
        if self.selected + 1 < self.matched.len() {
            self.selected += 1;
        }
    }

    pub fn toggle_debug(&mut self) {
        self.debug_enabled = !self.debug_enabled;
    }

    pub fn refresh_matches(&mut self) {
        self.matched = self.engine.search(&self.query, SearchScope::All);
        self.selected = 0;
    }

    pub fn set_engine(&mut self, engine: SpotlightEngine) {
        self.engine = engine;
    }
}