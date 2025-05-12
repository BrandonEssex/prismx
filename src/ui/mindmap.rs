// src/ui/mindmap.rs

use ratatui::layout::Rect;
use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui::text::{Line, Span};
use ratatui::Frame;

use crate::node_tree::NodeTree;
use crate::node::Node;

pub fn render_mindmap(frame: &mut Frame<'_>, area: Rect) {
    let block = Block::default()
        .title("Mindmap")
        .borders(Borders::ALL);

    let lines = vec![
        Line::from(Span::raw("Mindmap view placeholder")),
        Line::from(Span::raw("Rendering nodes will be added soon.")),
    ];

    let paragraph = Paragraph::new(lines).block(block);

    frame.render_widget(paragraph, area);
}