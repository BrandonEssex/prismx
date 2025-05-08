use crate::spotlight::engine::{SearchResult, SpotlightEngine};

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
            matched: Vec::new(),
            selected: 0,
            debug_enabled: false,
            engine: SpotlightEngine::new(),
        }
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
        self.matched = self.engine.search(&self.query);
        self.selected = 0;
    }

    pub fn selected(&self) -> Option<&SearchResult> {
        self.matched.get(self.selected)
    }

    pub fn engine_mut(&mut self) -> &mut SpotlightEngine {
        &mut self.engine
    }

    pub fn engine(&self) -> &SpotlightEngine {
        &self.engine
    }
}