use crate::spotlight::engine::{SpotlightEngine, SearchResult};

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

    pub fn close(&mut self) {
        self.query.clear();
        self.matched.clear();
        self.selected = 0;
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

    fn refresh_matches(&mut self) {
        self.matched = self.engine.search(&self.query);
        self.selected = 0;
    }

    pub fn is_active(&self) -> bool {
        !self.query.is_empty() || !self.matched.is_empty()
    }
}