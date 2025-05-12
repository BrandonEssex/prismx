// src/command_bar.rs

use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui::text::{Line, Span};
use ratatui::style::Style;
use ratatui::Frame;

pub fn render_command_bar(frame: &mut Frame<'_>, area: Rect, command: &str) {
    let block = Block::default().title("Command").borders(Borders::ALL);

    let paragraph = Paragraph::new(vec![Line::from(Span::raw(command))])
        .block(block)
        .style(Style::default());

    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3)].as_ref())
        .split(area);

    frame.render_widget(paragraph, layout[0]);
}
