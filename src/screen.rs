use ratatui::{Frame};
use crate::{
    actions::Action,
    state::AppState,
    spotlight::SpotlightModule,
};

pub struct Screen {
    spotlight: SpotlightModule,
}

impl Screen {
    pub fn new() -> Self {
        Screen {
            spotlight: SpotlightModule::new(),
        }
    }

    pub fn handle_event(
        &mut self,
        _evt: &crossterm::event::Event,
        action: Option<Action>,
        _state: &mut AppState,
    ) -> bool {
        match action {
            Some(Action::Quit) => return false,
            Some(a) => {
                self.spotlight.handle_action(a, _state);
            }
            _ => {}
        }

        true
    }

    pub fn draw(&mut self, f: &mut Frame) {
        self.spotlight.render(f);
    }
}