// src/dashboard.rs

use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui::text::{Line, Span};
use ratatui::style::Style;
use ratatui::Frame;

pub fn render_dashboard(frame: &mut Frame<'_>, area: Rect) {
    let block = Block::default()
        .borders(Borders::ALL)
        .title("PrismX Dashboard");

    let content = vec![
        Line::from("• View: Project Summary"),
        Line::from("• Status: All systems go"),
        Line::from("• Plugins: Loaded & Active"),
    ];

    let paragraph = Paragraph::new(content).block(block).style(Style::default());

    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(1)].as_ref())
        .split(area);

    frame.render_widget(paragraph, layout[0]);
}