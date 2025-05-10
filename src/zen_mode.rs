// FINAL FULL FILE DELIVERY
// Filename: /src/zen_mode.rs

use ratatui::{
    backend::Backend,
    layout::Rect,
    style::{Color, Style},
    text::{Span, Line},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::config::ZenConfig;

#[derive(Debug, Default)]
pub struct ZenModeState {
    pub active: bool,
    pub fade_delay: u64,
    pub autosave_secs: u64,
    pub scratchpad_path: String,
}

impl ZenModeState {
    pub fn new(config: &ZenConfig) -> Self {
        Self {
            active: false,
            fade_delay: config.title_fade_delay_secs,
            autosave_secs: config.autosave_interval_secs,
            scratchpad_path: config.scratchpad_path.clone(),
        }
    }

    pub fn render(&self, f: &mut Frame<'_>, area: Rect) {
        let block = Block::default()
            .title("Zen Mode")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::LightMagenta));

        let content = vec![
            Line::from(Span::styled(
                "Minimal focus mode. Press Ctrl+Z to toggle.",
                Style::default().fg(Color::Gray),
            )),
            Line::from(Span::raw(format!("Autosave: every {}s", self.autosave_secs))),
        ];

        let para = Paragraph::new(content).block(block);
        f.render_widget(para, area);
    }
}