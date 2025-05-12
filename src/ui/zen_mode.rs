// src/ui/zen_mode.rs

use ratatui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui::Frame;

pub fn render_zen_mode(frame: &mut Frame<'_>, area: Rect) {
    let block = Block::default().title("Zen Mode").borders(Borders::ALL);

    let lines = vec![
        Line::from(Span::styled("Stay focused.", Style::default().add_modifier(Modifier::BOLD))),
        Line::from("You are in Zen Mode. Press Esc to exit."),
    ];

    let paragraph = Paragraph::new(lines)
        .block(block)
        .alignment(Alignment::Center);

    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(1)].as_ref())
        .split(area);

    frame.render_widget(paragraph, layout[0]);
}