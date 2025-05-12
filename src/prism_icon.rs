// src/prism_icon.rs

use ratatui::layout::Rect;
use ratatui::style::{Color, Style};
use ratatui::text::Span;
use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui::Frame;

pub fn render_prism_icon(frame: &mut Frame<'_>, area: Rect, context: &str) {
    let block = Block::default().borders(Borders::ALL).title("PrismX");

    let symbol = match context {
        "zen" => Span::styled("X", Style::default().fg(Color::Green)),
        "work" => Span::styled("X", Style::default().fg(Color::Blue)),
        "personal" => Span::styled("X", Style::default().fg(Color::Magenta)),
        "budget" => Span::styled("X", Style::default().fg(Color::Red)),
        _ => Span::styled("X", Style::default().fg(Color::White)),
    };

    let paragraph = Paragraph::new(symbol).block(block);

    frame.render_widget(paragraph, area);
}