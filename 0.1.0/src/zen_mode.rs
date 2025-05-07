use std::time::{Duration, Instant};
use std::path::PathBuf;
use crate::scratchpad::Scratchpad;
use crate::config::ZenConfig;
use crate::logger::init_logging;

#[derive(Debug, PartialEq)]
pub enum ZenModeState {
    Inactive,
    Active {
        title_shown: bool,
        last_fade: Instant,
        scratchpad_path: PathBuf,
        config: ZenConfig,
    },
}

impl ZenModeState {
    pub fn toggle(current: &mut ZenModeState, config: ZenConfig) {
        match current {
            ZenModeState::Inactive => {
                init_logging(); // or log_zen("Zen Mode ACTIVATED");
                let path = config.scratchpad_path();
                *current = ZenModeState::Active {
                    title_shown: false,
                    last_fade: Instant::now(),
                    scratchpad_path: path,
                    config,
                };
            }
            ZenModeState::Active { .. } => {
                *current = ZenModeState::Inactive;
            }
        }
    }

    pub fn should_fade_title(&self) -> bool {
        match self {
            ZenModeState::Active { last_fade, config, .. } => {
                last_fade.elapsed() >= config.title_fade_delay
            }
            _ => false,
        }
    }

    pub fn render_active_ui(&self, frame: &mut ratatui::Frame, scratchpad: &Scratchpad) {
        use ratatui::widgets::{Block, Paragraph, Borders};
        use ratatui::layout::{Layout, Constraint, Direction};
        use ratatui::style::{Style, Modifier};

        if let ZenModeState::Active { title_shown, .. } = self {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(2)
                .constraints([
                    Constraint::Length(1),
                    Constraint::Min(1),
                ])
                .split(frame.size());

            if *title_shown {
                let title = Paragraph::new("Zen Mode – Scratchpad")
                    .style(Style::default().add_modifier(Modifier::ITALIC));
                frame.render_widget(title, chunks[0]);
            }

            let content = Paragraph::new(scratchpad.get_buffer())
                .block(Block::default().borders(Borders::NONE));
            frame.render_widget(content, chunks[1]);
        }
    }
}