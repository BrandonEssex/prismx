// src/status_bar.rs

use ratatui::layout::Rect;
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui::Frame;

pub fn render_status_bar(frame: &mut Frame<'_>, area: Rect, status: &str) {
    let block = Block::default().borders(Borders::ALL).title("Status");

    let line = Line::from(vec![
        Span::styled("● ", Style::default().fg(Color::Green)),
        Span::raw(status),
    ]);

    let paragraph = Paragraph::new(vec![line])
        .block(block)
        .style(Style::default().add_modifier(Modifier::ITALIC));

    frame.render_widget(paragraph, area);
}