use crate::actions::Action;
use crate::spotlight::SpotlightModule;
use crate::state::AppState;
use ratatui::Frame;

pub struct Screen {
    pub spotlight: SpotlightModule,
}

impl Screen {
    pub fn new() -> Self {
        Screen {
            spotlight: SpotlightModule::new(),
        }
    }

    pub fn handle_event(
        &mut self,
        _evt: crossterm::event::Event,
        action: Option<Action>,
        _state: &mut AppState,
    ) -> bool {
        match action {
            Some(Action::ToggleSpotlight) => {
                self.spotlight.toggle();
            }
            _ => {}
        }

        true
    }

    pub fn draw(&mut self, f: &mut Frame) {
        self.spotlight.render(f);
    }
}