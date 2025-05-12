// src/ui/mindmap.rs

use ratatui::layout::Rect;
use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui::text::{Line};
use ratatui::Frame;

use crate::node_tree::NodeTree;

pub fn render_mindmap(frame: &mut Frame<'_>, area: Rect, tree: &NodeTree) {
    let block = Block::default()
        .title("Mindmap")
        .borders(Borders::ALL);

    let mut lines = Vec::new();

    for root_id in &tree.root_ids {
        if let Some(root) = tree.get_node(root_id) {
            lines.push(Line::from(format!("• {}", root.title)));
            for child_id in &root.children {
                if let Some(child) = tree.get_node(child_id) {
                    lines.push(Line::from(format!("  └─ {}", child.title)));
                }
            }
        }
    }

    let paragraph = Paragraph::new(lines).block(block);
    frame.render_widget(paragraph, area);
}