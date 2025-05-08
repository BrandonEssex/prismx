use crate::scratchpad::Scratchpad;
use crate::config::ZenConfig;
use crate::util::logger::log_zen;
use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Modifier, Style},
    text::Span,
    widgets::{Block, Borders, Paragraph},
    Frame,
};
use std::time::Instant;

#[derive(Debug, Clone)]
pub enum ZenModeState {
    Inactive,
    Active {
        title_shown: bool,
        last_fade: Instant,
        scratchpad_path: std::path::PathBuf,
        config: ZenConfig,
    },
}

impl ZenModeState {
    pub fn toggle(current: &mut ZenModeState, config: ZenConfig) {
        match current {
            ZenModeState::Inactive => {
                log_zen("Zen Mode ACTIVATED");
                let path = config.scratchpad_path();
                *current = ZenModeState::Active {
                    title_shown: false,
                    last_fade: Instant::now(),
                    scratchpad_path: path,
                    config,
                };
            }
            ZenModeState::Active { .. } => {
                log_zen("Zen Mode DEACTIVATED");
                *current = ZenModeState::Inactive;
            }
        }
    }

    pub fn render_active_ui(&self, frame: &mut Frame, scratchpad: &Scratchpad) {
        if let ZenModeState::Active { title_shown, .. } = self {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(2)
                .constraints([Constraint::Length(1), Constraint::Min(1)])
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