use ratatui::{
    backend::Backend,
    layout::Rect,
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Paragraph},
    text::{Line, Span},
    Frame,
};
use crate::state::{AppState, DockLayout};
use crate::beamx::style_for_mode;

pub fn render_favorites_dock<B: Backend>(f: &mut Frame<B>, area: Rect, state: &mut AppState) {
    if !state.favorite_dock_enabled {
        return;
    }
    let mut favorites = state.favorite_entries();

    let theme = style_for_mode(&state.mode);
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
        let y = area.height.saturating_sub(height + 3);

        let border = Block::default().borders(Borders::ALL).style(base_style);
        f.render_widget(border, Rect::new(x - 1, y - 1, width, height));

        let spans: Vec<Span> = favorites
            .iter()
            .enumerate()
            .flat_map(|(i, e)| {
                let style = if Some(i) == state.dock_focus_index {
                    Style::default().fg(Color::LightCyan).add_modifier(Modifier::REVERSED)
                } else {
                    base_style
                };
                vec![Span::styled(e.icon.to_string(), style), Span::raw("  ")]
            })
            .collect();
        let line = Line::from(spans);
        f.render_widget(Paragraph::new(line), Rect::new(x, y, width - 2, 1));
    } else {
        if favorites.is_empty() {
            return;
        }
        let base_y = area.height.saturating_sub(favorites.len() as u16 + 2);
        f.render_widget(Paragraph::new("\\__").style(base_style), Rect::new(0, base_y - 1, 3, 1));
        for (i, entry) in favorites.iter().enumerate() {
            let gy = base_y + i as u16;
            let style = if Some(i) == state.dock_focus_index {
                Style::default().fg(Color::LightCyan).add_modifier(Modifier::REVERSED)
            } else {
                base_style
            };
            let line = format!("{} |", entry.icon);
            f.render_widget(Paragraph::new(line).style(style), Rect::new(0, gy, 5, 1));
        }
        let bottom_y = base_y + favorites.len() as u16;
        let underscore_len = area.width.saturating_sub(3) as usize;
        let bottom_line = format!("  |{}", "_".repeat(underscore_len));
        f.render_widget(Paragraph::new(bottom_line).style(base_style), Rect::new(0, bottom_y, area.width, 1));
    }
}
