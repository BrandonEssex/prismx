// src/dashboard.rs

use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui::text::{Span, Spans};
use ratatui::style::Style;
use ratatui::Frame;

pub fn render_dashboard(frame: &mut Frame<'_>, area: Rect) {
    let block = Block::default()
        .borders(Borders::ALL)
        .title("PrismX Dashboard");

    let content = vec![
        Spans::from(vec![Span::raw("• View: Project Summary")]),
        Spans::from(vec![Span::raw("• Status: All systems go")]),
        Spans::from(vec![Span::raw("• Plugins: Loaded & Active")]),
    ];

    let paragraph = Paragraph::new(content)
        .block(block)
        .style(Style::default());

    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(1)].as_ref())
        .split(area);

    frame.render_widget(paragraph, layout[0]);
}