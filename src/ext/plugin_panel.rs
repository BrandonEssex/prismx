// src/ext/plugin_panel.rs

use ratatui::layout::Rect;
use ratatui::style::{Color, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui::Frame;

pub fn render_plugin_panel(frame: &mut Frame<'_>, area: Rect, plugin_name: &str, status: &str) {
    let block = Block::default()
        .title(format!("Plugin Panel: {}", plugin_name))
        .borders(Borders::ALL);

    let content = vec![
        Line::from(format!("Status: {}", status)),
        Line::from(Span::styled("Active Plugin Panel", Style::default().fg(Color::Cyan))),
        Line::from("...custom rendering in progress..."),
    ];

    let paragraph = Paragraph::new(content).block(block);

    frame.render_widget(paragraph, area);
}