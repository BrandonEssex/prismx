use ratatui::{
    layout::Rect,
    text::{Line, Span},
    widgets::{Paragraph, Block, Borders},
    Frame,
};

use crate::state::AppState;

pub fn render_status_bar(f: &mut Frame<'_>, area: Rect, _state: &AppState) {
    let line = Line::from(vec![
        Span::raw("[ "),
        Span::raw("q: Quit "),
        Span::raw("] [ "),
        Span::raw("h: Help "),
        Span::raw("] [ "),
        Span::raw("/: Launcher "),
        Span::raw("] [ "),
        Span::raw("Tab: Sidebar "),
        Span::raw("] [ "),
        Span::raw("Esc: Hide "),
        Span::raw("]"),
    ]);

    let block = Paragraph::new(line).block(Block::default().borders(Borders::ALL).title("Status"));

    f.render_widget(block, area);
}