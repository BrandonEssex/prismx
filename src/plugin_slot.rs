// src/plugin_slot.rs

use ratatui::layout::Rect;
use ratatui::text::{Span, Line};
use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui::Frame;

pub fn render_plugin_slot(frame: &mut Frame<'_>, area: Rect, plugin_name: &str) {
    let block = Block::default()
        .title(format!("Plugin: {}", plugin_name))
        .borders(Borders::ALL);

    let lines = vec![
        Line::from("This is a plugin-rendered panel."),
        Line::from("Future output will go here."),
    ];

    let paragraph = Paragraph::new(lines).block(block);
    frame.render_widget(paragraph, area);
}
