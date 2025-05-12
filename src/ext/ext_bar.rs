// src/ext/ext_bar.rs

use ratatui::layout::Rect;
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui::Frame;

pub fn render_ext_bar(frame: &mut Frame<'_>, area: Rect, ext_name: &str) {
    let block = Block::default()
        .borders(Borders::ALL)
        .title("Extension Bar");

    let content = vec![
        Line::from(Span::styled(
            format!("Loaded: {}", ext_name),
            Style::default().fg(Color::Green).add_modifier(Modifier::BOLD),
        )),
        Line::from("Press 'Esc' to hide."),
    ];

    let paragraph = Paragraph::new(content).block(block);

    frame.render_widget(paragraph, area);
}