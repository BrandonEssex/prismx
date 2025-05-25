use ratatui::{
    backend::Backend,
    layout::Rect,
    style::Style,
    widgets::{Block, Borders, Paragraph},
    Frame,
};
use crate::state::{AppState, FavoriteEntry, DockLayout};
use crate::beamx::style_for_mode;

pub fn render_favorites_dock<B: Backend>(f: &mut Frame<B>, area: Rect, state: &mut AppState) {
    if !state.favorite_dock_enabled {
        return;
    }
    let default_favorites = [
        ("⚙️", "/settings"),
        ("📬", "/triage"),
        ("💭", "/gemx"),
        ("🧘", "/zen"),
        ("🔍", "/spotlight"),
    ];

    let mut all: Vec<FavoriteEntry> = default_favorites
        .iter()
        .map(|&(icon, cmd)| FavoriteEntry { icon, command: cmd })
        .chain(state.plugin_favorites.iter().cloned())
        .take(5)
        .collect();

    if state.mode == "gemx" && all.len() >= 3 {
        all[2].icon = "💬";
    }
    if state.mode == "triage" || state.show_triage {
        if all.len() >= 2 {
            all[1].icon = "📫";
        }
    }

    let favorites = &mut all[..];

    let theme = style_for_mode(&state.mode);
    let style = Style::default().fg(theme.border_color);

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

        let border = Block::default().borders(Borders::ALL).style(style);
        f.render_widget(border, Rect::new(x - 1, y - 1, width, height));

        let line: String = favorites.iter().map(|e| e.icon).collect::<Vec<_>>().join("  ");
        f.render_widget(Paragraph::new(line).style(style), Rect::new(x, y, width - 2, 1));
    } else {
        if favorites.is_empty() {
            return;
        }
        let base_y = area.height.saturating_sub(favorites.len() as u16 + 2);
        f.render_widget(Paragraph::new("\\__").style(style), Rect::new(0, base_y - 1, 3, 1));
        for (i, entry) in favorites.iter().enumerate() {
            let gy = base_y + i as u16;
            let line = format!("{} |", entry.icon);
            f.render_widget(Paragraph::new(line).style(style), Rect::new(0, gy, 5, 1));
        }
        let bottom_y = base_y + favorites.len() as u16;
        let underscore_len = area.width.saturating_sub(3) as usize;
        let bottom_line = format!("  |{}", "_".repeat(underscore_len));
        f.render_widget(Paragraph::new(bottom_line).style(style), Rect::new(0, bottom_y, area.width, 1));
    }
}
