pub mod actions;
pub mod debug;
pub mod engine;
pub mod favorites;
pub mod plugin;
pub mod state;
pub mod ui;

use ratatui::{backend::Backend, Frame};

use crate::actions::Action;
use crate::screen::AppState;

use self::state::SpotlightState;

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
            Action::MoveUp => self.state.move_up(),
            Action::MoveDown => self.state.move_down(),
            Action::Enter => {
                // placeholder: could be open
            }
            Action::Back => {}
            Action::Char('d') => self.state.toggle_debug(),
            Action::Char(c) => self.state.update_query(c),
            Action::Backspace => self.state.backspace(),
            _ => {}
        }

        true
    }

    pub fn render<B: Backend>(&mut self, f: &mut Frame<B>) {
        ui::render_overlay(f, &mut self.state);
    }

    pub fn open(&mut self) {
        self.state.open();
    }

    pub fn is_active(&self) -> bool {
        true // placeholder: stateful toggle logic could be added
    }
}