mod engine;
mod state;
mod ui;
mod actions;
mod debug;
mod plugin;

use ratatui::Frame;
use crate::actions::Action;
use crate::screen::AppState;
use state::SpotlightState;

pub struct SpotlightModule {
    state: SpotlightState,
    is_active: bool,
}

impl SpotlightModule {
    pub fn new() -> Self {
        SpotlightModule {
            state: SpotlightState::new(),
            is_active: false,
        }
    }

    pub fn handle_action(&mut self, action: Action, state: &mut AppState) -> bool {
        if !self.is_active {
            return false;
        }

        match action {
            Action::Up => self.state.move_up(),
            Action::Down => self.state.move_down(),
            Action::Enter => {
                if let Some(result) = self.state.selected() {
                    println!("Selected: {}", result.title);
                    self.toggle();
                }
            }
            Action::Char('m') => actions::perform_move(&mut self.state),
            Action::Char('x') => actions::perform_delete(&mut self.state),
            Action::Char('e') => actions::perform_export(&mut self.state),
            Action::Char('f') => actions::toggle_favorite(&mut self.state),
            Action::Char(c) => self.state.update_query(c),
            Action::Backspace => self.state.backspace(),
            Action::Exit => self.toggle(),
            _ => {}
        }

        true
    }

    pub fn render(&mut self, f: &mut Frame) {
        if self.is_active {
            ui::render_overlay(f, &mut self.state);
        }
    }

    pub fn toggle(&mut self) {
        self.is_active = !self.is_active;
    }

    pub fn is_active(&self) -> bool {
        self.is_active
    }
}