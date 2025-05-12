// src/plugin_slot.rs

use ratatui::layout::Rect;
use ratatui::text::{Span, Spans};
use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui::Frame;

pub fn render_plugin_slot(frame: &mut Frame<'_>, area: Rect, plugin_name: &str) {
    let block = Block::default()
        .title(format!("Plugin: {}", plugin_name))
        .borders(Borders::ALL);

    let spans = vec![
        Spans::from(Span::raw("This is a plugin-rendered panel.")),
        Spans::from(Span::raw("Future output will go here.")),
    ];

    let paragraph = Paragraph::new(spans).block(block);

    frame.render_widget(paragraph, area);
}