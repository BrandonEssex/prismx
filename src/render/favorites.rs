use ratatui::{
    backend::Backend,
    layout::Rect,
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Paragraph, Clear},
    Frame,
};
use crate::state::{AppState, DockLayout, FavoriteEntry};
use crate::config::theme::ThemeConfig;
use crate::ui::animate;
use std::time::{SystemTime, UNIX_EPOCH};

pub fn render_favorites_dock<B: Backend>(f: &mut Frame<B>, area: Rect, state: &mut AppState) {
    if !state.favorite_dock_enabled {
        return;
    }

    let favorites = state.favorite_entries();
    state.dock_entry_bounds.clear();

    let theme = state.beam_style_for_mode(&state.mode);
    let base_style = Style::default().fg(theme.border_color);
    let accent = ThemeConfig::load().accent_color();
    let tick = if std::env::var("PRISMX_TEST").is_ok() {
        0
    } else {
        (SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis()
            / 300) as u64
    };

    let horizontal = state.favorite_dock_layout == DockLayout::Horizontal;
    let height = if horizontal { 3 } else { favorites.len() as u16 + 2 };
    let width = if horizontal {
        favorites.len() as u16 * 3 + 2
    } else {
        6
    };

    if horizontal {
        let x = area.x + 1;
        // shift dock up slightly so icons never overlap the status bar
        let y = area.height.saturating_sub(height + 4);

        let border = Block::default().borders(Borders::ALL).style(base_style);
        f.render_widget(border, Rect::new(x - 1, y - 1, width, height));
        for (i, entry) in favorites.iter().enumerate() {
            let gx = x + i as u16 * 3;
            let rect = Rect::new(gx, y, 2, 1);
            let mut style = base_style;
            if state.favorite_focus_index == Some(i) {
                style = if ThemeConfig::load().dock_pulse() && state.dock_pulse_frames > 0 {
                    animate::shimmer(accent, tick).add_modifier(Modifier::REVERSED)
                } else {
                    Style::default().fg(accent).add_modifier(Modifier::REVERSED)
                };
            }
            f.render_widget(Paragraph::new(entry.icon).style(style), rect);
            state.dock_entry_bounds.push((rect, entry.command.to_string()));
        }

    } else {
        if favorites.is_empty() {
            return;
        }
        // leave extra padding above the footer
        let base_y = area.height.saturating_sub(favorites.len() as u16 + 3);
        f.render_widget(Paragraph::new("\\__").style(base_style), Rect::new(0, base_y - 1, 3, 1));
        for (i, entry) in favorites.iter().enumerate() {
            let gy = base_y + i as u16;
            let mut style = if Some(i) == state.dock_focus_index {
                Style::default().fg(Color::LightCyan).add_modifier(Modifier::REVERSED)
            } else {
                base_style
            };
            if state.favorite_focus_index == Some(i) {
                style = if ThemeConfig::load().dock_pulse() && state.dock_pulse_frames > 0 {
                    animate::shimmer(accent, tick).add_modifier(Modifier::REVERSED)
                } else {
                    Style::default().fg(accent).add_modifier(Modifier::REVERSED)
                };
            }
            let line = format!("{} |", entry.icon);
            let rect = Rect::new(0, gy, 5, 1);
            f.render_widget(Paragraph::new(line).style(style), rect);
            state.dock_entry_bounds.push((rect, entry.command.to_string()));
        }
        let bottom_y = base_y + favorites.len() as u16;
        let underscore_len = area.width.saturating_sub(3) as usize;
        let bottom_line = format!("  |{}", "_".repeat(underscore_len));
        f.render_widget(Paragraph::new(bottom_line).style(base_style), Rect::new(0, bottom_y, area.width, 1));
    }

    if state.dock_pulse_frames > 0 && std::env::var("PRISMX_TEST").is_err() {
        state.dock_pulse_frames -= 1;
    }

    render_dock_preview(f, area, state, &favorites, horizontal);
}

fn render_dock_preview<B: Backend>(
    f: &mut Frame<B>,
    area: Rect,
    state: &mut AppState,
    favorites: &[FavoriteEntry],
    horizontal: bool,
) {
    if let Some(idx) = state.dock_hover_index {
        if let Some((rect, _)) = state.dock_entry_bounds.get(idx) {
            if let Some(entry) = favorites.get(idx) {
                let text = format!("{} {}", entry.icon, entry.command);
                let width = text.len() as u16 + 2;
                let x;
                let y;
                if horizontal {
                    x = rect.x.saturating_sub(1);
                    y = rect.y.saturating_sub(3);
                } else {
                    x = rect.right() + 1;
                    y = rect.y;
                }
                let w = width.min(area.width.saturating_sub(x));
                let h = 3u16;
                let rect = Rect::new(x.min(area.right().saturating_sub(w)), y.max(area.top()), w, h);
                let block = Block::default().borders(Borders::ALL).style(Style::default().bg(Color::Black).fg(Color::White));
                f.render_widget(Clear, rect);
                f.render_widget(block, rect);
                f.render_widget(Paragraph::new(text), Rect::new(rect.x + 1, rect.y + 1, rect.width.saturating_sub(2), 1));
            }
        }
    }
}
