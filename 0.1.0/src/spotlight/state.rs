use crate::tui::framework::ScreenContext;
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

    pub fn activate_selected(&mut self, ctx: &mut ScreenContext) {
        if let Some(selected) = self.matched.get(self.selected) {
            ctx.open_by_uid(&selected.uid);
            self.close();
        }
    }

    pub fn toggle_debug(&mut self) {
        self.debug_enabled = !self.debug_enabled;
    }

    pub fn toggle_favorite(&mut self) {
        // Placeholder: integrate with favorites
    }

    pub fn queue_move(&mut self) {
        // Placeholder: trigger shard change UI
    }

    pub fn queue_delete(&mut self) {
        // Placeholder: show confirm dialog
    }

    pub fn queue_export(&mut self) {
        // Placeholder: write .md file from UID
    }

    fn refresh_matches(&mut self) {
        self.matched = self.engine.search(&self.query, self.scope.clone());
        self.selected = 0;
    }

    pub fn set_engine(&mut self, engine: SpotlightEngine) {
        self.engine = engine;
    }
}