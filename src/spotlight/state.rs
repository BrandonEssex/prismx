use super::engine::{SearchResult, SpotlightEngine};

pub struct SpotlightState {
    pub is_active: bool,
    pub query: String,
    pub matched: Vec<SearchResult>,
    pub selected: usize,
    pub debug_enabled: bool,
    pub engine: SpotlightEngine,
}

impl SpotlightState {
    pub fn new() -> Self {
        SpotlightState {
            is_active: false,
            query: String::new(),
            matched: Vec::new(),
            selected: 0,
            debug_enabled: false,
            engine: SpotlightEngine::new(),
        }
    }

    pub fn open(&mut self) {
        self.is_active = true;
        self.query.clear();
        self.refresh_matches();
    }

    pub fn close(&mut self) {
        self.is_active = false;
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

    pub fn update_query(&mut self, c: char) {
        self.query.push(c);
        self.refresh_matches();
    }

    pub fn backspace(&mut self) {
        self.query.pop();
        self.refresh_matches();
    }

    pub fn activate_selected(&mut self) {
        // Placeholder
        self.close();
    }

    pub fn toggle_debug(&mut self) {
        self.debug_enabled = !self.debug_enabled;
    }

    pub fn toggle_favorite(&mut self) {
        // Placeholder
    }

    pub fn queue_move(&mut self) {
        // Placeholder
    }

    pub fn queue_delete(&mut self) {
        // Placeholder
    }

    pub fn queue_export(&mut self) {
        // Placeholder
    }

    fn refresh_matches(&mut self) {
        self.matched = self.engine.search(&self.query);
        self.selected = 0;
    }
}