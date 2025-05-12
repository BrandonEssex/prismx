// src/ext/dashboard_slot.rs

use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::text::Line;
use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui::Frame;

pub fn render_dashboard_slot(frame: &mut Frame<'_>, area: Rect, title: &str, data: &[&str]) {
    let block = Block::default()
        .borders(Borders::ALL)
        .title(title);

    let content: Vec<Line> = data.iter().map(|line| Line::from(*line)).collect();

    let paragraph = Paragraph::new(content).block(block);

    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(1)].as_ref())
        .split(area);

    frame.render_widget(paragraph, layout[0]);
}