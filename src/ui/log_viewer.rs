// src/ui/log_viewer.rs

use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::Style;
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui::Frame;

pub fn render_log_viewer(frame: &mut Frame<'_>, area: Rect) {
    let block = Block::default()
        .title("Log Viewer")
        .borders(Borders::ALL);

    let log_lines = vec![
        Line::from(Span::raw("2025-05-11 12:00:00 - System started")),
        Line::from(Span::raw("2025-05-11 12:00:01 - Plugin loaded: scratchpad")),
        Line::from(Span::raw("2025-05-11 12:00:02 - Active view: Dashboard")),
    ];

    let paragraph = Paragraph::new(log_lines).block(block).style(Style::default());

    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(1)].as_ref())
        .split(area);

    frame.render_widget(paragraph, layout[0]);
}
