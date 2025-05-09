use ratatui::{
    layout::{Alignment, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Line},
    widgets::{Block, Borders, Paragraph},
    Frame,
};
use crate::scratchpad::Scratchpad;
use crate::config::Config;

pub struct ZenModeState {
    pub enabled: bool,
    pub breath_phase: usize,
    pub scratchpad: Scratchpad,
    pub autosave_path: String,
}

impl ZenModeState {
    pub fn new(config: &Config) -> Self {
        Self {
            enabled: false,
            breath_phase: 0,
            scratchpad: Scratchpad::new(),
            autosave_path: config.zen_mode.scratchpad_path.clone(),
        }
    }

    pub fn toggle(&mut self) {
        self.enabled = !self.enabled;
        self.breath_phase = 0;
    }

    pub fn tick(&mut self) {
        if self.enabled {
            self.breath_phase = (self.breath_phase + 1) % 8;
            self.autosave();
        }
    }

    pub fn autosave(&self) {
        use std::fs::write;
        if let Err(e) = write(&self.autosave_path, &self.scratchpad.content) {
            log::warn!("Zen autosave failed: {e}");
        }
    }

    pub fn render<B>(&self, frame: &mut Frame<'_>, area: Rect)
    where
        B: ratatui::backend::Backend,
    {
        if self.enabled {
            let breath_label = match self.breath_phase {
                0..=3 => "Inhale",
                4..=5 => "Hold",
                _ => "Exhale",
            };

            let para = Paragraph::new(Line::from(vec![Span::styled(
                breath_label,
                Style::default().fg(Color::Blue).add_modifier(Modifier::BOLD),
            )]))
            .alignment(Alignment::Center)
            .block(Block::default().title("Breathing").borders(Borders::ALL));

            frame.render_widget(para, area);
        }
    }
}