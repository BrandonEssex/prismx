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
    let mut all = vec![
        FavoriteEntry { label: "⚙️", mode: "settings", bounds: Rect::default() },
        FavoriteEntry { label: "📬", mode: "triage", bounds: Rect::default() },
        FavoriteEntry { label: "💭", mode: "gemx", bounds: Rect::default() },
        FavoriteEntry { label: "🧘", mode: "zen", bounds: Rect::default() },
        FavoriteEntry { label: "🔍", mode: "spotlight", bounds: Rect::default() },
    ];

    if state.mode == "gemx" && all.len() >= 3 {
        all[2].label = "💬";
    }
    if state.mode == "triage" || state.show_triage {
        if all.len() >= 2 {
            all[1].label = "📫";
        }
    }

    let limit = state.favorite_dock_limit.min(all.len());
    let favorites = &mut all[..limit];

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

        let line: String = favorites.iter().map(|e| e.label).collect::<Vec<_>>().join("  ");
        f.render_widget(Paragraph::new(line).style(style), Rect::new(x, y, width - 2, 1));
        for (i, entry) in favorites.iter_mut().enumerate() {
            entry.bounds = Rect::new(x + (i as u16 * 3), y, 2, 1);
        }
    } else {
        if favorites.is_empty() {
            return;
        }
        let base_y = area.height.saturating_sub(favorites.len() as u16 + 2);
        f.render_widget(Paragraph::new("\\__").style(style), Rect::new(0, base_y - 1, 3, 1));
        for (i, entry) in favorites.iter_mut().enumerate() {
            let gy = base_y + i as u16;
            let line = format!("{} |", entry.label);
            f.render_widget(Paragraph::new(line).style(style), Rect::new(0, gy, 5, 1));
            entry.bounds = Rect::new(0, gy, 2, 1);
        }
        let bottom_y = base_y + favorites.len() as u16;
        let underscore_len = area.width.saturating_sub(3) as usize;
        let bottom_line = format!("  |{}", "_".repeat(underscore_len));
        f.render_widget(Paragraph::new(bottom_line).style(style), Rect::new(0, bottom_y, area.width, 1));
    }

    state.favorite_entries = favorites.to_vec();
}
