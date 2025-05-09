use crate::storage::inbox_storage::InboxState;
use crate::scratchpad::Scratchpad;
use crate::config::Config;

pub struct AppState {
    pub config: Config,
    pub scratchpad: Scratchpad,
    pub inbox: InboxState,
    pub should_quit: bool,
}

impl AppState {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            scratchpad: Scratchpad::new(),
            inbox: InboxState::new(),
            should_quit: false,
        }
    }

    pub fn is_running(&self) -> bool {
        !self.should_quit
    }

    pub fn quit(&mut self) {
        self.should_quit = true;
    }
}