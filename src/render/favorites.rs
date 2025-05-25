use ratatui::{backend::Backend, layout::Rect, style::{Style, Color}, widgets::Paragraph, Frame};
use crate::state::{AppState, FavoriteEntry};

pub fn render_favorites_dock<B: Backend>(f: &mut Frame<B>, area: Rect, state: &mut AppState) {
    let mut entries = vec![
        FavoriteEntry { label: "⚙️", mode: "settings", bounds: Rect::default() },
        FavoriteEntry { label: "📬", mode: "triage", bounds: Rect::default() },
        FavoriteEntry { label: "💭", mode: "gemx", bounds: Rect::default() },
    ];

    if state.mode == "gemx" {
        entries[2].label = "💬";
    }
    if state.mode == "triage" || state.show_triage {
        entries[1].label = "📫";
    }

    let x = area.x + 1;
    let base_y = area.height.saturating_sub(6);
    let style = Style::default().fg(Color::Cyan);

    f.render_widget(Paragraph::new("|__").style(style), Rect::new(x, base_y, 12, 1));
    for (i, entry) in entries.iter_mut().enumerate() {
        let y = base_y + 1 + i as u16;
        let line = match i {
            0 => format!(" {}\\", entry.label),
            1 => format!(" {} \\", entry.label),
            _ => format!(" {}   \\____", entry.label),
        };
        f.render_widget(Paragraph::new(line).style(style), Rect::new(x, y, 12, 1));
        entry.bounds = Rect::new(x, y, 4, 1);
    }

    state.favorite_entries = entries;
}
