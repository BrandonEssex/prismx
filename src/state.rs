use crate::mindmap_state::MindmapState;
use crate::storage::inbox_storage::InboxState;
use crate::config::Config;

#[derive(Debug, Default)]
pub struct AppState {
    pub config: Config,
    pub scratchpad: crate::scratchpad::Scratchpad,
    pub inbox: InboxState,
    pub mindmap: MindmapState,
    pub running: bool,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            config: Config::default(),
            scratchpad: crate::scratchpad::Scratchpad::default(),
            inbox: InboxState::default(),
            mindmap: MindmapState::new(),
            running: true,
        }
    }

    pub fn is_running(&self) -> bool {
        self.running
    }

    pub fn quit(&mut self) {
        self.running = false;
    }
}