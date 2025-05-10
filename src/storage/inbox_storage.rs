use serde::{Deserialize, Serialize};
use crate::state::AppState;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InboxState {
    pub tasks: Vec<String>, // adjust this to real task type
    pub context_open: bool,
}

impl InboxState {
    pub fn new() -> Self {
        Self {
            tasks: Vec::new(),
            context_open: false,
        }
    }
}