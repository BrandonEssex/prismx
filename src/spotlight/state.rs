use super::engine::{SearchResult, SpotlightEngine};
use super::plugin::SearchScope;

#[derive(Default)]
pub struct SpotlightState {
    pub query: String,
    pub matched: Vec<SearchResult>,
    pub selected: usize,
    pub debug_enabled: bool,
    engine: SpotlightEngine,
}

impl SpotlightState {
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

    fn refresh_matches(&mut self) {
        self.matched = self.engine.search(&self.query, SearchScope::All);
        self.selected = 0;
    }

    pub fn set_engine(&mut self, engine: SpotlightEngine) {
        self.engine = engine;
    }
}