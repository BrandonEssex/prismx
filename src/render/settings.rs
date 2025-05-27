// src/render/settings.rs
use ratatui::{
    backend::Backend,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Wrap},
    Frame,
};

use crate::config::theme::ThemeConfig;
use crate::settings::{SettingToggle, SETTING_TOGGLES};
use crate::state::AppState;

pub fn render_settings<B: Backend>(f: &mut Frame<B>, area: Rect, state: &AppState) {
    let theme = ThemeConfig::load();
    let mut lines: Vec<Line> = Vec::new();

    for (i, toggle) in SETTING_TOGGLES.iter().enumerate() {
        let enabled = (toggle.is_enabled)(state);
        let selected = i == state.settings_focus_index % SETTING_TOGGLES.len();
        let mut label = toggle.label.to_string();

        if label.starts_with("Theme Preset") {
            label = format!("Theme Preset: {}", crate::settings::current_theme());
        } else if label.starts_with("BeamX Theme") {
            label = format!("BeamX Theme: {}", state.beamx_panel_theme);
        }

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
            Span::styled(prefix, style),
            Span::styled(format!("{} {}", check, toggle.icon), style),
            Span::styled(format!(" {}", label), style),
        ]));

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
    let height = (lines.len() + 2).min(area.height as usize - 1) as u16;

    let x = area.x + (area.width.saturating_sub(width)) / 2;
    let y = area.y + (area.height.saturating_sub(height)) / 2;

    let block = Block::default().title("Settings").borders(Borders::ALL);
    let content = Paragraph::new(lines).block(block).wrap(Wrap { trim: false });

    f.render_widget(content, Rect::new(x, y, width, height));
}
