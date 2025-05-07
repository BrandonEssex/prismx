use super::engine::{SearchResult, SpotlightEngine};
use super::plugin::SearchScope;

#[derive(Default)]
pub struct SpotlightState {
    pub is_active: bool,
    pub query: String,
    pub matched: Vec<SearchResult>,
    pub selected: usize,
    pub scope: SearchScope,
    pub debug_enabled: bool,
    engine: SpotlightEngine,
}

impl SpotlightState {
    pub fn open(&mut self) {
        self.is_active = true;
        self.query.clear();
        self.matched.clear();
        self.selected = 0;
    }

    pub fn close(&mut self) {
        self.is_active = false;
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

    pub fn activate_selected(&mut self) {
        if let Some(selected) = self.matched.get(self.selected) {
            // Integration hook: `ctx.open_by_uid(&selected.uid)`
            log::info!("Activated UID: {}", selected.uid);
            self.close();
        }
    }

    pub fn toggle_debug(&mut self) {
        self.debug_enabled = !self.debug_enabled;
    }

    pub fn toggle_favorite(&mut self) {
        // Implement integration with favorites module
    }

    pub fn queue_move(&mut self) {
        // Implement shard reassignment
    }

    pub fn queue_delete(&mut self) {
        // Trigger deletion
    }

    pub fn queue_export(&mut self) {
        // Export content to .md
    }

    fn refresh_matches(&mut self) {
        self.matched = self.engine.search(&self.query, self.scope.clone());
        self.selected = 0;
    }

    pub fn set_engine(&mut self, engine: SpotlightEngine) {
        self.engine = engine;
    }
}