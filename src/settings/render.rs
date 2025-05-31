use ratatui::{
    backend::Backend,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Wrap},
    Frame,
};

use crate::state::AppState;
use crate::config::theme::ThemeConfig;
use super::{layout::settings_area, toggle::{SETTING_TOGGLES, SettingCategory}};
use crate::state::{ShortcutOverlayMode, HeartbeatMode, LayoutStyle};
use crate::theme::previews::preview_line;

pub fn render_settings<B: Backend>(f: &mut Frame<B>, area: Rect, state: &mut AppState) {
    let theme = ThemeConfig::load();
    let mut lines: Vec<Line> = Vec::new();
    let header_style = Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD);

    let mut last_cat: Option<SettingCategory> = None;
    let mut toggle_lines: Vec<usize> = Vec::new();

    for (i, t) in SETTING_TOGGLES.iter().enumerate() {
        if last_cat.map_or(true, |c| c != t.category) {
            if last_cat.is_some() {
                lines.push(Line::default());
            }
            lines.push(Line::from(Span::styled(t.category.name(), header_style)));
            last_cat = Some(t.category);
        }

        let enabled = (t.is_enabled)(state);
        let selected = i == state.settings_focus_index % SETTING_TOGGLES.len();
        let mut label = t.label.to_string();
        if t.label == "Font Style" {
            label = format!("Font Style: {}", state.font_style.name());
        } else if t.label == "Shortcut Overlay" {
            label = format!("Shortcut Overlay: {}", match state.shortcut_overlay {
                ShortcutOverlayMode::Full => "Full",
                ShortcutOverlayMode::Contextual => "Contextual",
            });
        } else if t.label == "Heartbeat" {
            label = format!("Heartbeat: {}", match state.heartbeat_mode {
                HeartbeatMode::Pulse => "pulse",
                HeartbeatMode::Test => "test",
                HeartbeatMode::Silent => "silent",
            });
        } else if t.label == "Layout Style" {
            label = format!(
                "Layout Style: {}",
                match state.layout_style {
                    LayoutStyle::Compact => "Compact",
                    LayoutStyle::Relaxed => "Relaxed",
                }
            );
        }

        let status = if matches!(t.label, "Shortcut Overlay" | "Heartbeat" | "Font Style" | "Layout Style") {
            "".to_string()
        } else if enabled { "[✔]".into() } else { "[ ]".into() };

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

        let line_index = lines.len();
        lines.push(Line::from(vec![
            Span::styled(prefix.to_string(), style),
            Span::styled(format!("{} {} {}", status, t.icon, label), style),
        ]));
        toggle_lines.push(line_index);
    }

    lines.push(Line::default());
    lines.push(preview_line(state.font_style, state.settings_beam_color));

    let rect = settings_area(area, &lines);

    state.settings_toggle_bounds.clear();
    for (idx, line_idx) in toggle_lines.iter().enumerate() {
        let y = rect.y + 1 + (*line_idx as u16);
        state.settings_toggle_bounds.push((Rect::new(rect.x + 1, y, rect.width - 2, 1), idx));
    }

    let block = Block::default().title("Settings").borders(Borders::ALL);
    let content = Paragraph::new(lines).block(block).wrap(Wrap { trim: false });

    f.render_widget(content, rect);
}
