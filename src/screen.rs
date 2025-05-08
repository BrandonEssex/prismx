use crate::state::AppState;
use crate::spotlight::SpotlightModule;
use ratatui::{backend::Backend, Frame};

pub struct Screen {
    config: crate::config::Config,
    spotlight: SpotlightModule,
}

impl Screen {
    pub fn new(config: crate::config::Config, spotlight: SpotlightModule) -> Self {
        Self { config, spotlight }
    }

    pub fn draw<B: Backend>(&mut self, _f: &mut Frame<'_>, _state: &mut AppState) {
        log::info!("Drawing screen... (stub)");
    }
}