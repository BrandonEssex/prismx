// src/theme.rs

use ratatui::style::{Color, Style};

#[derive(Debug, Clone)]
pub struct Theme {
    pub background: Color,
    pub foreground: Color,
    pub accent: Color,
    pub warning: Color,
}

impl Default for Theme {
    fn default() -> Self {
        Theme {
            background: Color::Black,
            foreground: Color::White,
            accent: Color::Cyan,
            warning: Color::Red,
        }
    }
}

impl Theme {
    pub fn to_style(&self) -> Style {
        Style::default().fg(self.foreground).bg(self.background)
    }

    pub fn warning_style(&self) -> Style {
        Style::default().fg(self.warning)
    }

    pub fn accent_style(&self) -> Style {
        Style::default().fg(self.accent)
    }
}