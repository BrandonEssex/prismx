mod debug;
mod engine;
mod state;
mod ui;

use crate::actions::Action;
use crate::screen::AppState;
use ratatui::Frame;
use state::SpotlightState;

pub struct SpotlightModule {
    state: SpotlightState,
}

impl SpotlightModule {
    pub fn new() -> Self {
        SpotlightModule {
            state: SpotlightState::new(),
        }
    }

    pub fn handle_action(&mut self, action: Action, _state: &mut AppState) -> bool {
        match action {
            Action::Up => self.state.move_up(),
            Action::Down => self.state.move_down(),
            Action::Char('d') => self.state.toggle_debug(),
            Action::Char(c) => self.state.update_query(c),
            Action::Backspace => self.state.backspace(),
            Action::Enter => {} // placeholder for activation
            Action::Quit => return false,
            _ => {}
        }
        true
    }

    pub fn render(&mut self, f: &mut Frame) {
        ui::render_overlay(f, &self.state);
        if self.state.debug_enabled {
            debug::render_debug_overlay(f, &self.state);
        }
    }
}