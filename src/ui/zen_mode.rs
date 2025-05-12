// src/ui/zen_mode.rs

use ratatui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui::Frame;

pub fn render_zen_mode(frame: &mut Frame<'_>, area: Rect) {
    let block = Block::default()
        .title("Zen Mode")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Green));

    let lines = vec![
        Line::from(Span::styled(
            "Stay focused.",
            Style::default()
                .fg(Color::Green)
                .bg(Color::Black)
                .add_modifier(Modifier::BOLD),
        )),
        Line::from(Span::styled(
            "You are in Zen Mode. Press Esc to exit.",
            Style::default().fg(Color::Green),
        )),
    ];

    let paragraph = Paragraph::new(lines)
        .block(block)
        .alignment(Alignment::Center)
        .style(Style::default().bg(Color::Black));

    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(1)].as_ref())
        .split(area);

    frame.render_widget(paragraph, layout[0]);
}