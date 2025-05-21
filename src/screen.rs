use std::collections::HashMap;
use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, Paragraph};

use crate::layout::{Coords, layout_nodes};
use crate::node::{NodeID, NodeMap};

/// Renders nodes based on 2D layout produced by `layout_nodes`
pub fn render_gemx<B: Backend>(
    f: &mut Frame<B>,
    area: Rect,
    nodes: &NodeMap,
    root_id: NodeID,
    selected: Option<NodeID>,
) {
    let block = Block::default()
        .title("Gemx")
        .borders(Borders::ALL);
    f.render_widget(block, area);

    let drawn_at = layout_nodes(nodes, root_id, 2, 1);

    for (&node_id, &Coords { x, y }) in &drawn_at {
        if y as u16 >= area.height {
            continue;
        }

        let node = &nodes[&node_id];
        let style = if Some(node_id) == selected {
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD | Modifier::UNDERLINED)
        } else {
            Style::default().fg(Color::White)
        };

        let label = if Some(node_id) == selected {
            format!("> {}", node.label)
        } else {
            format!("  {}", node.label)
        };

        let para = Paragraph::new(label).style(style);
        f.render_widget(para, Rect::new(x, y, area.width.saturating_sub(x), 1));
    }
}
