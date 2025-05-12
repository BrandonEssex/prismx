// src/ui/command_bar.rs

use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui::text::{Line, Span};
use ratatui::style::{Style, Modifier};
use ratatui::Frame;

pub fn render_command_bar(frame: &mut Frame<'_>, area: Rect, buffer: &str) {
    let block = Block::default()
        .title("Command")
        .borders(Borders::ALL);

    let line = Line::from(Span::styled(
        format!("> {}", buffer),
        Style::default().add_modifier(Modifier::BOLD),
    ));

    let paragraph = Paragraph::new(vec![line])
        .block(block)
        .style(Style::default());

    frame.render_widget(paragraph, area);
}