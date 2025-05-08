pub mod actions;
mod debug;
mod engine;
mod plugin;
pub mod state;
pub mod ui;

use crate::actions::Action;
use crate::spotlight::state::SpotlightState;
use ratatui::Frame;
use crate::screen::AppState;

pub struct SpotlightModule {
    pub state: SpotlightState,
}

impl SpotlightModule {
    pub fn new() -> Self {
        Self {
            state: SpotlightState::new(),
        }
    }

    pub fn handle_action(&mut self, action: Action, _state: &mut AppState) -> bool {
        match action {
            Action::Up => self.state.move_up(),
            Action::Down => self.state.move_down(),
            Action::Enter => self.state.activate_selected(),
            Action::Backspace => self.state.backspace(),
            Action::Char('d') => self.state.toggle_debug(),
            Action::Char('m') => self.state.queue_move(),
            Action::Char('x') => self.state.queue_delete(),
            Action::Char('e') => self.state.queue_export(),
            Action::Char('f') => self.state.toggle_favorite(),
            Action::Char(c) => self.state.update_query(c),
            Action::Exit => self.state.close(),
            _ => {}
        }
        true
    }

    pub fn render(&mut self, f: &mut Frame) {
        if self.state.is_active() {
            ui::render_overlay(f, &mut self.state);
        }
    }

    pub fn toggle(&mut self) {
        if self.state.is_active() {
            self.state.close();
        } else {
            self.state.open();
        }
    }

    pub fn is_active(&self) -> bool {
        self.state.is_active()
    }
}