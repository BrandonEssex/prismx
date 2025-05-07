use crate::actions::Action;
use crate::state::AppState;
use crate::view_triage::draw_triage_view;
use crate::zen_mode::ZenModeState;
use crate::scratchpad::Scratchpad;
use crate::config::ZenConfig;
use crate::spotlight::SpotlightModule;
use crate::mode::Mode;

use ratatui::Frame;
use ratatui::backend::Backend;
use crossterm::event::Event;

pub struct Screen {
    pub mode: Mode,
    spotlight: SpotlightModule,
    zen_state: ZenModeState,
    scratchpad: Option<Scratchpad>,
    zen_config: ZenConfig,
}

impl Screen {
    pub fn new() -> Self {
        let zen_config = ZenConfig::default();
        Self {
            mode: Mode::Normal,
            spotlight: SpotlightModule::new(),
            zen_state: ZenModeState::Inactive,
            scratchpad: None,
            zen_config,
        }
    }

    pub fn handle_event(
        &mut self,
        evt: Event,
        action: Option<Action>,
        state: &mut AppState,
    ) -> bool {
        match action {
            Some(Action::Quit) => return false,

            Some(Action::ToggleZenMode) => {
                ZenModeState::toggle(&mut self.zen_state, self.zen_config.clone());
                self.mode = Mode::Zen;
                if let ZenModeState::Active { scratchpad_path, .. } = &self.zen_state {
                    self.scratchpad = Some(Scratchpad::new(scratchpad_path.clone()));
                } else {
                    self.scratchpad = None;
                }
            }

            Some(Action::ToggleTriage) => {
                self.mode = if self.mode == Mode::Triage {
                    Mode::Normal
                } else {
                    Mode::Triage
                };
            }

            Some(Action::ToggleSpotlight) => {
                self.spotlight.toggle();
                self.mode = if self.spotlight.is_active() {
                    Mode::Spotlight
                } else {
                    Mode::Normal
                };
            }

            Some(a) if self.spotlight.is_active() => {
                self.spotlight.handle_action(a);
            }

            _ => {}
        }

        true
    }

    pub fn update(&mut self, _state: &mut AppState) {
        if let Some(scratchpad) = self.scratchpad.as_mut() {
            scratchpad.autosave_if_needed();
        }
    }

    pub fn draw(&mut self, f: &mut Frame) {
        match self.mode {
            Mode::Zen => {
                if let Some(scratchpad) = &self.scratchpad {
                    self.zen_state.render_active_ui(f, scratchpad);
                }
            }
            Mode::Triage => {
                draw_triage_view(f, &AppState::default());
            }
            Mode::Spotlight => {
                self.spotlight.render(f);
            }
            Mode::Normal => {
                let area = f.size();
                let paragraph = ratatui::widgets::Paragraph::new("Welcome to PrismX")
                    .block(ratatui::widgets::Block::default().title("PrismX").borders(ratatui::widgets::Borders::ALL));
                f.render_widget(paragraph, area);
            }
        }
    }
}