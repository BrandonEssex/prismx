use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, Paragraph};
use crate::layout::{layout_nodes, Coords, GEMX_HEADER_HEIGHT};
use crate::node::{NodeID, NodeMap};
use crate::state::AppState;
use std::collections::HashMap;

pub fn render_gemx<B: Backend>(f: &mut Frame<B>, area: Rect, state: &AppState) {
    let block = Block::default()
        .title(if state.auto_arrange { "Gemx [Auto-Arrange]" } else { "Gemx" })
        .borders(Borders::ALL);
    f.render_widget(block, area);

    // // ✅ Always print the structure for diagnostics
    // println!("=== NODES AND CHILDREN ===");
    // for (id, node) in &state.nodes {
    //     println!("Node {} → parent: {:?}, children: {:?}", id, node.parent, node.children);
    // }

    let roots = if let Some(drill_root) = state.drawing_root {
        vec![drill_root]
    } else {
        state.root_nodes.clone()
    };

    let mut drawn_at = HashMap::new();
    if state.auto_arrange {
        let mut row = GEMX_HEADER_HEIGHT + 1;
        for &root_id in &roots {
            let layout = layout_nodes(&state.nodes, root_id, row, area.width as i16);
            let max_y = layout.values().map(|c| c.y).max().unwrap_or(row);
            drawn_at.extend(layout);
            row = max_y.saturating_add(3);
        }
    } else {
        fn collect(nodes: &NodeMap, id: NodeID, out: &mut HashMap<NodeID, Coords>) {
            if let Some(n) = nodes.get(&id) {
                out.insert(id, Coords { x: n.x, y: n.y });
                if !n.collapsed {
                    for child in &n.children {
                        collect(nodes, *child, out);
                    }
                }
            }
        }
        for &root_id in &roots {
            collect(&state.nodes, root_id, &mut drawn_at);
        }
    }

    for (&node_id, &Coords { x, y }) in &drawn_at {
        let draw_x = (x - state.scroll_x).max(0) as u16;
        let draw_y = (y - state.scroll_y).max(0) as u16;

        if draw_y >= area.height {
            continue;
        }

        let node = &state.nodes[&node_id];
        let is_selected = Some(node_id) == state.selected;

        let parent_glyph = if let Some(parent_id) = node.parent {
            let parent_y = drawn_at.get(&parent_id).map(|c| c.y).unwrap_or(y - 1);
            if parent_y < y { Some("│ ") } else { Some("  ") }
        } else { None };

        let elbow_glyph = if let Some(parent_id) = node.parent {
            let siblings = &state.nodes[&parent_id].children;
            if let Some(last) = siblings.last() {
                if *last == node_id { Some("└─") } else { Some("├─") }
            } else { None }
        } else { None };

        let mut label = String::new();
        if is_selected { label.push_str("> "); } else { label.push_str("  "); }
        if let Some(glyph) = parent_glyph { label.push_str(glyph); }
        if let Some(elbow) = elbow_glyph { label.push_str(elbow); }
        label.push_str(&node.label);
        if state.link_map.get(&node_id).and_then(|v| v.first()).is_some() {
            label.push_str(" 📎");
        }

        let width = label.len().min((area.width - draw_x) as usize);

        let style = if is_selected {
            Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD | Modifier::UNDERLINED)
        } else {
            Style::default().fg(Color::White)
        };

        let para = Paragraph::new(label).style(style);
        f.render_widget(para, Rect::new(draw_x, draw_y, width as u16, 1));
    }

    // Draw arrows
    for (source, targets) in &state.link_map {
        for target in targets {
            if let (Some(&Coords { x: sx, y: sy }), Some(&Coords { x: tx, y: ty })) =
                (drawn_at.get(source), drawn_at.get(target)) {
                if sy == ty {
                    let arrow = if sx < tx { "→" } else { "←" };
                    let mid = (sx + tx) / 2;
                    let draw_mid = (mid - state.scroll_x).max(0) as u16;
                    let draw_sy = (sy - state.scroll_y).max(0) as u16;
                    if draw_sy < area.height && draw_mid < area.width {
                        let para = Paragraph::new(arrow);
                        f.render_widget(para, Rect::new(draw_mid, draw_sy, 1, 1));
                    }
                }
            }
        }
    }
}
