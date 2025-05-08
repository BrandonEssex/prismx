use crate::state::AppState;
use crate::actions::Action;
use crate::spotlight::SpotlightModule;
use ratatui::{backend::Backend, Terminal, Frame};

pub struct Screen {
    config: crate::config::Config,
    spotlight: SpotlightModule,
}

impl Screen {
    pub fn new(config: crate::config::Config, spotlight: SpotlightModule) -> Self {
        Self { config, spotlight }
    }

    pub fn draw<B: Backend>(&mut self, terminal: &mut Frame<'_>, state: &mut AppState)pub fn draw<B: Backend>(&mut self, terminal: &mut Frame<'_>, state: &mut AppState) {
        // Placeholder: real render logic omitted
        log::info!("Drawing PrismX TUI...");
    }
}
