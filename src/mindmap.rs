// src/mindmap.rs

use ratatui::layout::Rect;
use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui::text::{Spans, Span};
use ratatui::Frame;

use crate::node_tree::NodeTree;
use crate::node::Node;

pub fn render_mindmap(frame: &mut Frame<'_>, area: Rect, tree: &NodeTree) {
    let block = Block::default()
        .title("Mindmap")
        .borders(Borders::ALL);

    let lines: Vec<Spans> = tree.root_ids.iter().filter_map(|id| {
        tree.get_node(id).map(|node| {
            Spans::from(Span::raw(format!("• {}", node.title)))
        })
    }).collect();

    let paragraph = Paragraph::new(lines).block(block);

    frame.render_widget(paragraph, area);
}