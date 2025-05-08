mod actions;
mod debug;
mod engine;
mod favorites;
mod plugin;
mod state;
mod ui;

use crate::actions::Action;
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

    pub fn handle_action(&mut self, action: Action, _state: &mut crate::state::AppState) -> bool {
        match action {
            Action::Up => self.state.move_up(),
            Action::Down => self.state.move_down(),
            Action::Enter => self.state.activate_selected(),
            Action::Back => self.state.close(),
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
        if self.state.is_open() {
            ui::render_overlay(f, &mut self.state);
        }
    }

    pub fn toggle(&mut self) {
        if self.state.is_open() {
            self.state.close();
        } else {
            self.state.open();
        }
    }

    pub fn is_active(&self) -> bool {
        self.state.is_open()
    }
}