// src/render/settings.rs
use ratatui::{
    backend::Backend,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Wrap},
    Frame,
};

use crate::state::AppState;
use crate::render::{render_settings, settings_len, SETTING_TOGGLES};
use crate::config::theme::ThemeConfig;

pub fn render_settings<B: Backend>(f: &mut Frame<B>, area: Rect, state: &AppState) {
    let theme = ThemeConfig::load();
    let mut lines: Vec<Line> = Vec::new();

    for (i, t) in SETTING_TOGGLES.iter().enumerate() {
        let enabled = (t.is_enabled)(state);
        let selected = i == state.settings_focus_index % SETTING_TOGGLES.len();
        let label = t.label;

        let check = if enabled { "[x]" } else { "[ ]" };
        let prefix = if selected { "> " } else { "  " };

        let mut style = if selected {
            Style::default()
                .fg(theme.focus_outline())
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(Color::Gray)
        };
        if selected {
            style = style.bg(Color::Black);
        }

        lines.push(Line::from(vec![
            Span::styled(prefix.to_string(), style),
            Span::styled(format!("{} {} {}", check, t.icon, label), style),
        ]));

        // Spacer lines after specific entries
        if i == 2 || i == 3 {
            lines.push(Line::default());
        }
    }

    let content_width = lines
        .iter()
        .map(|l| l.width() as u16)
        .max()
        .unwrap_or(0)
        .saturating_add(4);

    let width = content_width.min(area.width);
    let height = lines.len() as u16 + 2;
    let x = area.x + (area.width.saturating_sub(width)) / 2;
    let y = area.y + (area.height.saturating_sub(height)) / 2;

    let block = Block::default().title("Settings").borders(Borders::ALL);
    let content = Paragraph::new(lines).block(block).wrap(Wrap { trim: false });

    f.render_widget(content, Rect::new(x, y, width, height));
}
