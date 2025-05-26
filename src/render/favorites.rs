use ratatui::{
    backend::Backend,
    layout::Rect,
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Paragraph},
    text::{Line, Span},
    Frame,
};
use crate::state::{AppState, DockLayout};

pub fn render_favorites_dock<B: Backend>(f: &mut Frame<B>, area: Rect, state: &mut AppState) {
    if !state.favorite_dock_enabled {
        return;
    }

    let mut favorites = state.favorite_entries();
    state.dock_entry_bounds.clear();

    let theme = state.beam_style_for_mode(&state.mode);
    let base_style = Style::default().fg(theme.border_color);

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
            f.render_widget(Paragraph::new(entry.icon).style(base_style), rect);
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
            let style = if Some(i) == state.dock_focus_index {
                Style::default().fg(Color::LightCyan).add_modifier(Modifier::REVERSED)
            } else {
                base_style
            };
            let line = format!("{} |", entry.icon);
            let rect = Rect::new(0, gy, 5, 1);
            let style_entry = if state.favorite_focus_index == Some(i) {
                style.add_modifier(ratatui::style::Modifier::REVERSED)
            } else {
                style
            };
            f.render_widget(Paragraph::new(line).style(style_entry), rect);
            state.dock_entry_bounds.push((rect, entry.command.to_string()));
        }
        let bottom_y = base_y + favorites.len() as u16;
        let underscore_len = area.width.saturating_sub(3) as usize;
        let bottom_line = format!("  |{}", "_".repeat(underscore_len));
        f.render_widget(Paragraph::new(bottom_line).style(base_style), Rect::new(0, bottom_y, area.width, 1));
    }
}
