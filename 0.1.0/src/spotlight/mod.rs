pub mod actions;
pub mod debug;
pub mod engine;
pub mod favorites;
pub mod plugin;
pub mod state;
pub mod tests;
pub mod ui;

use crate::mode::Mode;
use crate::input::Action;
use crate::spotlight::state::SpotlightState;
use ratatui::Frame;

pub struct SpotlightModule {
    state: SpotlightState,
}

impl SpotlightModule {
    pub fn new() -> Self {
        SpotlightModule {
            state: SpotlightState::default(),
        }
    }

    pub fn handle_action(&mut self, action: Action) -> bool {
        if !self.state.is_active {
            return false;
        }

        match action {
            Action::MoveUp => self.state.move_up(),
            Action::MoveDown => self.state.move_down(),
            Action::Submit => self.state.activate_selected(),
            Action::Exit => self.state.close(),
            Action::Char('m') => self.state.queue_move(),
            Action::Char('x') => self.state.queue_delete(),
            Action::Char('e') => self.state.queue_export(),
            Action::Char('f') => self.state.toggle_favorite(),
            Action::Ctrl('d') => self.state.toggle_debug(),
            Action::Input(c) => self.state.update_query(c),
            Action::Backspace => self.state.backspace(),
            _ => {}
        }

        true
    }

    pub fn render(&mut self, f: &mut Frame) {
        if self.state.is_active {
            crate::spotlight::ui::render_overlay(f, &mut self.state);
        }
    }

    pub fn toggle(&mut self) {
        if self.state.is_active {
            self.state.close();
        } else {
            self.state.open();
        }
    }

    pub fn is_active(&self) -> bool {
        self.state.is_active
    }
}