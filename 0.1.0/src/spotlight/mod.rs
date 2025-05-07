pub mod actions;
pub mod debug;
pub mod engine;
pub mod favorites;
pub mod plugin;
pub mod state;
pub mod ui;

use crate::input::Action;
use crate::screen::AppState;
use ratatui::Frame;
use ratatui::backend::Backend;
use state::SpotlightState;

pub struct SpotlightModule {
    state: SpotlightState,
}

impl SpotlightModule {
    pub fn new() -> Self {
        SpotlightModule {
            state: SpotlightState::default(),
        }
    }

    pub fn handle_action(&mut self, action: Action, _state: &mut AppState) -> bool {
        match action {
            Action::Char(c) => self.state.update_query(c),
            Action::Backspace => self.state.backspace(),
            Action::MoveUp => self.state.move_up(),
            Action::MoveDown => self.state.move_down(),
            _ => {}
        }
        true
    }

    pub fn render<B: Backend>(&mut self, f: &mut Frame<B>) {
        ui::render_overlay(f, &mut self.state);
    }

    pub fn set_engine(&mut self, engine: engine::SpotlightEngine) {
        self.state.set_engine(engine);
    }
}