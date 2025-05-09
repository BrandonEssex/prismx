// FINAL FULL FILE DELIVERY
// Filename: /src/storage/inbox_storage.rs

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InboxState {
    pub tasks: Vec<String>,
    pub context_open: bool,
}

impl InboxState {
    pub fn toggle_context(&mut self) {
        self.context_open = !self.context_open;
    }
}