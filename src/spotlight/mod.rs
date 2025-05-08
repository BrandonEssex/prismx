use crate::actions::Action;
use crate::spotlight::state::SpotlightState;
use ratatui::{Frame, backend::Backend};

pub struct SpotlightModule {
    pub state: SpotlightState,
}

impl SpotlightModule {
    pub fn new() -> Self {
        SpotlightModule {
            state: SpotlightState::new(),
        }
    }

    pub fn handle_action(&mut self, action: Action, state: &mut crate::state::AppState) -> bool {
        match action {
            Action::Enter => self.state.activate_selected(state),
            Action::Backspace => self.state.backspace(),
            Action::Exit => self.state.close(),
            _ => {}
        }
        true
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

    pub fn render<B: Backend>(&mut self, f: &mut Frame<B>) {
        if self.state.is_active() {
            self.state.render(f);
        }
    }
}