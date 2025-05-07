use crate::state::AppState;
use super::engine::{SearchResult, SpotlightEngine};

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

    pub fn toggle_debug(&mut self) {
        self.debug_enabled = !self.debug_enabled;
    }

    pub fn close(&mut self) {
        self.query.clear();
        self.matched.clear();
        self.selected = 0;
        self.debug_enabled = false;
    }

    pub fn activate_selected(&mut self, _state: &mut AppState) {
        if let Some(selected) = self.matched.get(self.selected) {
            let _uid = &selected.uid;
            self.close();
        }
    }

    fn refresh_matches(&mut self) {
        self.matched = self.engine.search(&self.query, super::plugin::SearchScope::All);
        self.selected = 0;
    }
}