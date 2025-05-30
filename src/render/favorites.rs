use ratatui::{
    backend::Backend,
    layout::Rect,
    style::{Color, Modifier, Style},
    widgets::{Paragraph, Clear},
    Frame,
};
use crate::state::{AppState, DockLayout, FavoriteEntry};
use crate::config::theme::ThemeConfig;
use crate::ui::animate;
use crate::ui::borders::draw_rounded_border;
use crate::theme::characters::{HORIZONTAL, VERTICAL};
use std::time::{SystemTime, UNIX_EPOCH};

pub fn render_favorites_dock<B: Backend>(f: &mut Frame<B>, area: Rect, state: &mut AppState) {
    if !state.favorite_dock_enabled {
        return;
    }

    let favorites = state.favorite_entries();
    state.dock_entry_bounds.clear();

    let theme = state.beam_style_for_mode(&state.mode);
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

    let y = area.y + area.height.saturating_sub(2);
    let mut x = area.x + 1;

    for (i, entry) in favorites.iter().enumerate() {
        let rect = Rect::new(x, y, 2, 1);
        let mut style = Style::default().fg(theme.status_color);
        if state.favorite_focus_index == Some(i) || state.dock_hover_index == Some(i) {
            style = if ThemeConfig::load().dock_pulse() && state.dock_pulse_frames > 0 {
                animate::shimmer(accent, tick).add_modifier(Modifier::REVERSED)
            } else {
                Style::default().fg(accent).add_modifier(Modifier::REVERSED)
            };
        }
        f.render_widget(Paragraph::new(entry.icon).style(style), rect);
        state.dock_entry_bounds.push((rect, entry.command.to_string()));
        x += 3;
    }

    if state.dock_pulse_frames > 0 && std::env::var("PRISMX_TEST").is_err() {
        state.dock_pulse_frames -= 1;
    }

    render_dock_preview(f, area, state, &favorites, true);
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
                let block_style = Style::default().bg(Color::Black).fg(Color::White);
                f.render_widget(Clear, rect);
                draw_rounded_border(f, rect, block_style);
                f.render_widget(Paragraph::new(text).style(block_style), Rect::new(rect.x + 1, rect.y + 1, rect.width.saturating_sub(2), 1));
            }
        }
    }
}
