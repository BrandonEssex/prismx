use crate::actions::Action;
use crate::state::AppState;
use crate::spotlight::SpotlightModule;
use ratatui::Frame;
use crossterm::event::Event;

pub struct Screen {
    spotlight: SpotlightModule,
}

impl Screen {
    pub fn new() -> Self {
        Self {
            spotlight: SpotlightModule::new(),
        }
    }

    pub fn handle_event(&mut self, _evt: Event, action: Option<Action>, state: &mut AppState) -> bool {
        match action {
            Some(Action::Quit) => return false,
            Some(act) => {
                self.spotlight.handle_action(act, state);
            }
            _ => {}
        }

        true
    }

    pub fn draw(&mut self, f: &mut Frame) {
        self.spotlight.render(f);
    }
}