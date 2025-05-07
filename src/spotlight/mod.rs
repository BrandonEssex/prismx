pub mod actions;
pub mod debug;
pub mod engine;
pub mod favorites;
pub mod plugin;
pub mod state;
pub mod ui;

use crate::actions::Action;
use crate::state::AppState;
use ratatui::Frame;
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

    pub fn handle_action(&mut self, action: Action, state: &mut AppState) -> bool {
        match action {
            Action::Char(c) => self.state.update_query(c),
            Action::Backspace => self.state.backspace(),
            Action::MoveUp => self.state.move_up(),
            Action::MoveDown => self.state.move_down(),
            Action::Enter => self.state.activate_selected(state),
            _ => {}
        }
        true
    }

    pub fn render(&mut self, f: &mut Frame) {
        ui::render_overlay(f, &mut self.state);
    }
}