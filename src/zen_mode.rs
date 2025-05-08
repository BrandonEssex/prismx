use crate::scratchpad::Scratchpad;
use crate::config::Config;

use log::info;
use ratatui::{
    layout::{Alignment, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

pub struct ZenModeState {
    pub enabled: bool,
    pub scratchpad: Scratchpad,
}

impl ZenModeState {
    pub fn new(config: &Config) -> Self {
        Self {
            enabled: false,
            scratchpad: Scratchpad::new(),
        }
    }

    pub fn toggle(&mut self) {
        self.enabled = !self.enabled;
        info!("Zen Mode toggled: {}", self.enabled);
    }

    pub fn render<B: ratatui::backend::Backend>(&self, frame: &mut Frame<B>, area: Rect) {
        if self.enabled {
            let block = Block::default()
                .title("Zen Mode")
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Blue));
            let paragraph = Paragraph::new(Spans::from(vec![
                Span::styled("Type here...", Style::default().add_modifier(Modifier::ITALIC)),
            ]))
            .block(block)
            .alignment(Alignment::Left);
            frame.render_widget(paragraph, area);
        }
    }
}
