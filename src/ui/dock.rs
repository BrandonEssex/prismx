use ratatui::{backend::Backend, style::{Color, Modifier, Style}, widgets::Paragraph, Frame};
use crate::ui::layout::Rect;
use crate::ui::borders::draw_rounded_border;
use crate::state::{AppState, DockLayout};
use crate::config::theme::ThemeConfig;
use crate::theme::characters::{HORIZONTAL, VERTICAL};

pub fn render_dock<B: Backend>(f: &mut Frame<B>, area: Rect, state: &mut AppState) {
    if !state.favorite_dock_enabled {
        return;
    }

    let favorites = state.favorite_entries();
    state.dock_entry_bounds.clear();

    let beam = state.beam_style_for_mode(&state.mode);
    let base_style = Style::default().fg(beam.border_color);
    let accent = ThemeConfig::load().accent_color();
    let horizontal = state.favorite_dock_layout == DockLayout::Horizontal;
    let height = if horizontal { 3 } else { favorites.len() as u16 + 2 };
    let width = if horizontal {
        favorites.len() as u16 * 3 + 2
    } else {
        6
    };

    if horizontal {
        let x = area.x + 1;
        // align closely with status bar
        let y = area.height.saturating_sub(height + 3);
        draw_rounded_border(f, Rect::new(x - 1, y - 1, width, height), base_style);
        for (i, entry) in favorites.iter().enumerate() {
            let gx = x + i as u16 * 3;
            let rect = Rect::new(gx, y, 2, 1);
            let mut style = base_style;
            if state.favorite_focus_index == Some(i) {
                style = Style::default().fg(accent).add_modifier(Modifier::REVERSED);
            }
            f.render_widget(Paragraph::new(entry.icon).style(style), rect);
            state.dock_entry_bounds.push((rect, entry.command.to_string()));
        }
    } else {
        if favorites.is_empty() {
            return;
        }
        let base_y = area.height.saturating_sub(favorites.len() as u16 + 3);
        f.render_widget(Paragraph::new(format!("{}{}", VERTICAL, HORIZONTAL)).style(base_style), Rect::new(0, base_y - 1, 2, 1));
        for (i, entry) in favorites.iter().enumerate() {
            let gy = base_y + i as u16;
            let mut style = if Some(i) == state.dock_focus_index {
                Style::default().fg(Color::LightCyan).add_modifier(Modifier::REVERSED)
            } else {
                base_style
            };
            if state.favorite_focus_index == Some(i) {
                style = Style::default().fg(accent).add_modifier(Modifier::REVERSED);
            }
            let line = format!("{} {}", entry.icon, VERTICAL);
            let rect = Rect::new(0, gy, 4, 1);
            f.render_widget(Paragraph::new(line).style(style), rect);
            state.dock_entry_bounds.push((rect, entry.command.to_string()));
        }
        let bottom_y = base_y + favorites.len() as u16;
        let underscore_len = area.width.saturating_sub(3) as usize;
        let bottom_line = format!("{}{}{}", VERTICAL, HORIZONTAL.repeat(underscore_len), VERTICAL);
        f.render_widget(Paragraph::new(bottom_line).style(base_style), Rect::new(0, bottom_y, area.width, 1));
    }

    if state.dock_pulse_frames > 0 && std::env::var("PRISMX_TEST").is_err() {
        state.dock_pulse_frames -= 1;
    }
}
