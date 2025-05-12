// src/sidebar.rs

use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui::text::{Line, Span};
use ratatui::style::Style;
use ratatui::Frame;

pub fn render_sidebar_panel(frame: &mut Frame<'_>, area: Rect, title: &str, content: &[&str]) {
    let block = Block::default()
        .title(title)
        .borders(Borders::ALL);

    let lines: Vec<Line> = content
        .iter()
        .map(|line| Line::from(Span::raw(*line)))
        .collect();

    let paragraph = Paragraph::new(lines)
        .block(block)
        .style(Style::default());

    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(1)].as_ref())
        .split(area);

    frame.render_widget(paragraph, layout[0]);
}