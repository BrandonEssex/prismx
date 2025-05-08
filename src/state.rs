use crate::config::Config;
use crate::inbox::InboxState;
use crate::scratchpad::Scratchpad;

pub struct AppState {
    pub config: Config,
    pub scratchpad: Scratchpad,
    pub inbox: InboxState,
    pub should_quit: bool,
}

impl AppState {
    pub fn new(config: Config) -> Self {
        Self {
            scratchpad: Scratchpad::new(),
            inbox: InboxState::default(),
            should_quit: false,
            config,
        }
    }

    pub fn is_running(&self) -> bool {
        !self.should_quit
    }

    pub fn quit(&mut self) {
        self.should_quit = true;
    }
}
