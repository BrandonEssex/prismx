use ratatui::{prelude::*, widgets::Paragraph};
use crate::node::{NodeID, NodeMap};
use crate::layout::engine::layout_vertical;
use crate::ui::lines::draw_line;

/// Render a simple mindmap using the vertical layout engine.
pub fn render<B: Backend>(
    f: &mut Frame<B>,
    area: Rect,
    nodes: &mut NodeMap,
    roots: &[NodeID],
    debug: bool,
) {
    for &root in roots {
        layout_vertical(nodes, root);
    }

    if debug {
        // Draw connections first
        let mut connections = Vec::new();
        let all_ids: Vec<NodeID> = nodes.keys().copied().collect();
        for id in all_ids {
            if let Some(node) = nodes.get(&id) {
                for child_id in &node.children {
                    if let Some(child) = nodes.get(child_id) {
                        connections.push(((node.x, node.y), (child.x, child.y)));
                    }
                }
            }
        }

        for ((sx, sy), (ex, ey)) in connections {
            let ox = area.x as i16;
            let oy = area.y as i16;
            draw_line(f, (sx + ox, sy + oy), (ex + ox, ey + oy));
        }
    }

    // Draw nodes
    for node in nodes.values() {
        let x = area.x as i16 + node.x;
        let y = area.y as i16 + node.y;
        if x >= 0 && y >= 0 && x < area.right() as i16 && y < area.bottom() as i16 {
            let rect = Rect::new(x as u16, y as u16, node.label.len() as u16, 1);
            f.render_widget(Paragraph::new(node.label.clone()), rect);
        }
    }
}
