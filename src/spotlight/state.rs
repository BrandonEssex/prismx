use crate::spotlight::engine::{SearchResult, SpotlightEngine};

#[derive(Debug)]
pub struct SpotlightState {
    pub query: String,
    pub matched: Vec<SearchResult>,
    pub selected: usize,
    pub debug_enabled: bool,
    pub engine: SpotlightEngine,
    active: bool,
}

impl SpotlightState {
    pub fn new() -> Self {
        Self {
            query: String::new(),
            matched: Vec::new(),
            selected: 0,
            debug_enabled: false,
            engine: SpotlightEngine::new(),
            active: false,
        }
    }

    pub fn is_active(&self) -> bool {
        self.active
    }

    pub fn open(&mut self) {
        self.active = true;
        self.query.clear();
        self.refresh_matches();
    }

    pub fn close(&mut self) {
        self.active = false;
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
        if let Some(item) = self.matched.get(self.selected) {
            println!("Activated item: {}", item.title);
        }
        self.close();
    }

    pub fn toggle_debug(&mut self) {
        self.debug_enabled = !self.debug_enabled;
    }

    pub fn toggle_favorite(&mut self) {
        if let Some(item) = self.matched.get(self.selected) {
            println!("Favorite toggled for UID: {}", item.title);
        }
    }

    pub fn queue_move(&mut self) {
        if let Some(item) = self.matched.get(self.selected) {
            println!("Move triggered for UID: {}", item.title);
        }
    }

    pub fn queue_delete(&mut self) {
        if let Some(item) = self.matched.get(self.selected) {
            println!("Delete triggered for UID: {}", item.title);
        }
    }

    pub fn queue_export(&mut self) {
        if let Some(item) = self.matched.get(self.selected) {
            println!("Export triggered for UID: {}", item.title);
        }
    }

    fn refresh_matches(&mut self) {
        self.matched = self.engine.search(&self.query);
        self.selected = 0;
    }
}