use ratatui::Frame;
use crate::input::{Action, Config};
use crate::zen_mode::ZenModeState;
use crate::scratchpad::Scratchpad;
use crate::config::ZenConfig;
use crate::state::AppState;
use crossterm::event::Event;

pub struct Screen {
    config: Config,
    zen_state: ZenModeState,
    scratchpad: Option<Scratchpad>,
    zen_config: ZenConfig,
}

impl Screen {
    pub fn new() -> Self {
        let config = Config;
        let zen_config = ZenConfig::default();

        Self {
            config,
            zen_state: ZenModeState::Inactive,
            scratchpad: None,
            zen_config,
        }
    }

    pub fn handle_event(&mut self, evt: Event, _state: &mut AppState) -> bool {
        if let Some(action) = self.config.map(evt.clone()) {
            match action {
                Action::Quit => return false,

                Action::ToggleZenMode => {
                    ZenModeState::toggle(&mut self.zen_state, self.zen_config.clone());
                    match &self.zen_state {
                        ZenModeState::Active { scratchpad_path, .. } => {
                            self.scratchpad = Some(Scratchpad::new(scratchpad_path.clone()));
                        }
                        ZenModeState::Inactive => {
                            self.scratchpad = None;
                        }
                    }
                }

                Action::OpenScratchpad => {
                    if !matches!(self.zen_state, ZenModeState::Active { .. }) {
                        ZenModeState::toggle(&mut self.zen_state, self.zen_config.clone());
                    }

                    if let ZenModeState::Active { scratchpad_path, .. } = &self.zen_state {
                        self.scratchpad = Some(Scratchpad::new(scratchpad_path.clone()));
                    }
                }

                _ => {}
            }
        }

        true
    }

    pub fn update(&mut self, _state: &mut AppState) {
        if let Some(scratchpad) = self.scratchpad.as_mut() {
            scratchpad.autosave_if_needed();
        }
    }

    pub fn draw(&mut self, frame: &mut Frame) {
        match &self.zen_state {
            ZenModeState::Active { .. } => {
                if let Some(scratchpad) = &self.scratchpad {
                    self.zen_state.render_active_ui(frame, scratchpad);
                }
            }
            _ => {
                self.render_standard_ui(frame);
            }
        }
    }

    fn render_standard_ui(&self, _frame: &mut Frame) {
        // Placeholder for normal UI rendering
    }
}