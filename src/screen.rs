use crate::actions::Action;
use crate::input::Config;
use crate::screen::AppState;
use crate::spotlight::SpotlightModule;
use ratatui::Frame;

pub struct Screen {
    pub config: Config,
    pub spotlight: SpotlightModule,
}

impl Screen {
    pub fn new(config: Config, spotlight: SpotlightModule) -> Self {
        Screen { config, spotlight }
    }

    pub fn handle_event(&mut self, _evt: crossterm::event::Event, action: Option<Action>, state: &mut AppState) -> bool {
        if let Some(action) = action {
            match action {
                Action::Quit => return false,
                Action::ToggleSpotlight => self.spotlight.toggle(),
                Action::Enter => self.spotlight.state.activate_selected(state),
                Action::Backspace => self.spotlight.state.backspace(),
                _ => {}
            }
        }
        true
    }

    pub fn draw<B: ratatui::backend::Backend>(&mut self, f: &mut Frame<B>) {
        self.spotlight.render(f);
    }
}