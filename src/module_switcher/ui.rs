use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame, backend::Backend,
};

use crate::state::AppState;

pub fn render_module_switcher<B: Backend>(f: &mut Frame<B>, area: Rect, _state: &AppState) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![Constraint::Length(3), Constraint::Min(0)])
        .split(area);

    let title = Paragraph::new(Line::from(vec![
        Span::styled("Module Switcher", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
    ])).block(Block::default().borders(Borders::ALL).title("Modules"));

    f.render_widget(title, chunks[0]);
}
