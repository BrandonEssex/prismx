use crate::actions::Action;
use crate::state::AppState;
use crate::spotlight::SpotlightModule;
use crate::view_triage::draw_triage_view;
use crate::mode::Mode;
use ratatui::{Frame};

pub struct Screen {
    pub mode: Mode,
    pub spotlight: SpotlightModule,
}

impl Screen {
    pub fn new() -> Self {
        Screen {
            mode: Mode::Normal,
            spotlight: SpotlightModule::new(),
        }
    }

    pub fn handle_event(
        &mut self,
        evt: crossterm::event::Event,
        action: Option<Action>,
        state: &mut AppState,
    ) -> bool {
        match action {
            Some(Action::Quit) => return false,
            Some(Action::ToggleTriage) => {
                self.mode = match self.mode {
                    Mode::Triage => Mode::Normal,
                    _ => Mode::Triage,
                };
            }
            Some(Action::ToggleSpotlight) => {
                self.mode = Mode::Spotlight;
                self.spotlight.toggle();
            }
            _ => {}
        }

        if self.mode == Mode::Spotlight {
            if let Some(a) = action {
                self.spotlight.handle_action(a, state);
            }
        }

        true
    }

    pub fn draw(&mut self, f: &mut Frame) {
        match self.mode {
            Mode::Triage => {
                // Assume you want to see Inbox content
                // This assumes state is available in the context, otherwise refactor
            }
            Mode::Spotlight => {
                self.spotlight.render(f);
            }
            Mode::Normal | Mode::Zen => {
                // Draw default view
            }
        }
    }
}