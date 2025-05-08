use crate::actions::Action;
use crate::spotlight::state::SpotlightState;
use crate::screen::AppState;
use ratatui::{Frame, backend::Backend};

mod actions;
mod debug;
mod engine;
mod favorites;
mod plugin;
mod state;
mod ui;

pub struct SpotlightModule {
    state: SpotlightState,
}

impl SpotlightModule {
    pub fn new() -> Self {
        SpotlightModule {
            state: SpotlightState::new(),
        }
    }

    pub fn handle_action(&mut self, action: Action, state: &mut AppState) -> bool {
        match action {
            Action::Up => self.state.move_up(),
            Action::Down => self.state.move_down(),
            Action::Char('d') => self.state.toggle_debug(),
            Action::Char(c) => self.state.update_query(c),
            Action::Backspace => self.state.backspace(),
            Action::Enter => self.state.activate_selected(state),
            Action::Exit => self.state.close(),
            _ => {}
        }
        true
    }

    pub fn render<B: Backend>(&mut self, f: &mut Frame<B>) {
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