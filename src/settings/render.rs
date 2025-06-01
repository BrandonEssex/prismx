use ratatui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Tabs, Wrap},
    Frame,
};

use crate::state::AppState;
use crate::config::theme::ThemeConfig;
use super::{
    layout::settings_area,
    toggle::{toggles_for_category, SettingCategory, SETTING_CATEGORIES},
};
use crate::state::{ShortcutOverlayMode, HeartbeatMode, LayoutStyle};
use crate::theme::previews::preview_line;

pub fn render_settings<B: Backend>(f: &mut Frame<B>, area: Rect, state: &mut AppState) {
    let theme = ThemeConfig::load();
    let category = SETTING_CATEGORIES[state.settings_selected_tab % SETTING_CATEGORIES.len()];
    let toggles = toggles_for_category(category);
    let mut lines: Vec<Line> = Vec::new();
    let mut toggle_lines: Vec<usize> = Vec::new();

    for (display_idx, (_idx, t)) in toggles.iter().enumerate() {
        let enabled = (t.is_enabled)(state);
        let selected = display_idx == state.settings_focus_index % toggles.len();
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
        } else if enabled {
            "[✔]".into()
        } else {
            "[ ]".into()
        };

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

    let mut layout_lines = Vec::with_capacity(lines.len() + 1);
    layout_lines.push(Line::default());
    layout_lines.extend(lines.iter().cloned());

    let rect = settings_area(area, &layout_lines);
    let layout_ok = layout_lines.len() as u16 + 2 <= area.height && rect.height >= 3;

    if layout_ok {
        let block = Block::default().title("Settings").borders(Borders::ALL);
        f.render_widget(block.clone(), rect);
        let inner = block.inner(rect);
        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(1), Constraint::Min(1)])
            .split(inner);

        let tab_titles: Vec<Line> = SETTING_CATEGORIES
            .iter()
            .map(|c| Line::from(c.name()))
            .collect();
        let tabs = Tabs::new(tab_titles)
            .select(state.settings_selected_tab)
            .highlight_style(
                Style::default()
                    .fg(theme.focus_outline())
                    .add_modifier(Modifier::BOLD),
            )
            .style(Style::default().fg(Color::Gray));
        f.render_widget(tabs, layout[0]);

        let content = Paragraph::new(lines).wrap(Wrap { trim: false });
        f.render_widget(content, layout[1]);

        state.settings_toggle_bounds.clear();
        for (idx, line_idx) in toggle_lines.iter().enumerate() {
            let y = layout[1].y + (*line_idx as u16);
            if y < layout[1].y + layout[1].height.saturating_sub(1) {
                state.settings_toggle_bounds.push((
                    Rect::new(layout[1].x, y, layout[1].width, 1),
                    idx,
                ));
            }
        }
    } else {
        crate::log_warn!("SETTINGS_LAYOUT_FAIL");
        let block = Block::default().title("Settings").borders(Borders::ALL);
        let empty = Paragraph::new("").block(block);
        f.render_widget(empty, rect);
        state.settings_toggle_bounds.clear();
    }
}
