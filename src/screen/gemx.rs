use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, Paragraph};
use crate::gemx::layout::{layout_nodes, Coords};
use crate::node::{NodeID, NodeMap};
use crate::state::AppState;
use std::collections::HashMap;

pub fn render_gemx<B: Backend>(f: &mut Frame<B>, area: Rect, state: &AppState) {
    let block = Block::default()
        .title(if state.auto_arrange { "Gemx [Auto-Arrange]" } else { "Gemx" })
        .borders(Borders::ALL);
    f.render_widget(block, area);

    let roots = if let Some(drill_root) = state.drawing_root {
        vec![drill_root]
    } else {
        state.root_nodes.clone()
    };

    let mut drawn_at = HashMap::new();
    let mut y = 1;

    if state.auto_arrange {
        for &root_id in &roots {
            let layout = layout_nodes(&state.nodes, root_id, 2, y);
            let max_y = layout.values().map(|c| c.y).max().unwrap_or(y);
            drawn_at.extend(layout);
            y = max_y.saturating_add(crate::gemx::layout::CHILD_SPACING_Y);
        }
    } else {
        fn collect(nodes: &NodeMap, id: NodeID, out: &mut HashMap<NodeID, Coords>, idx: &mut u16) {
            if let Some(n) = nodes.get(&id) {
                let (x, y) = if n.x == 0 && n.y == 0 {
                    let grid_x = (*idx % 10) * crate::gemx::layout::SIBLING_SPACING_X;
                    let grid_y = (*idx / 10) * crate::gemx::layout::CHILD_SPACING_Y;
                    *idx += 1;
                    (grid_x as u16, grid_y as u16)
                } else {
                    (n.x as u16, n.y as u16)
                };
                out.insert(id, Coords { x, y });
                if !n.collapsed {
                    for child in &n.children {
                        collect(nodes, *child, out, idx);
                    }
                }
            }
        }
        let mut idx = 0u16;
        for &root_id in &roots {
            collect(&state.nodes, root_id, &mut drawn_at, &mut idx);
        }
    }

    let zoom = state.zoom_scale;

    for (&node_id, &Coords { x, y }) in &drawn_at {
        let zx = ((x as f32) * zoom).round() as u16;
        let zy = ((y as f32) * zoom).round() as u16;
        if zy >= area.height {
            continue;
        }

        let node = &state.nodes[&node_id];
        let is_selected = Some(node_id) == state.selected;

        let parent_glyph = if let Some(parent_id) = node.parent {
            let parent_y = drawn_at.get(&parent_id).map(|c| c.y).unwrap_or(y.saturating_sub(1));
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

        let scroll_x = state.scroll_x.max(0) as u16;
        let width = label.len().min((area.width - zx.saturating_sub(scroll_x)) as usize);

        let style = if is_selected {
            Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD | Modifier::UNDERLINED)
        } else {
            Style::default().fg(Color::White)
        };

        let para = Paragraph::new(label).style(style);
        f.render_widget(para, Rect::new(zx.saturating_sub(scroll_x), zy, width as u16, 1));
    }

    // Draw link arrows between nodes (horizontal layout)
    for (source, targets) in &state.link_map {
        for target in targets {
            if let (Some(&Coords { x: sx, y: sy }), Some(&Coords { x: tx, y: ty })) =
                (drawn_at.get(source), drawn_at.get(target))
            {
                if sy == ty && sy < area.height {
                    let arrow = if sx < tx { "→" } else { "←" };
                    let mid = (((sx + tx) / 2) as f32 * zoom).round() as u16;
                    let sy = ((sy as f32) * zoom).round() as u16;
                    let scroll_x = state.scroll_x.max(0) as u16;
                    if mid >= scroll_x && mid < scroll_x + area.width {
                        let para = Paragraph::new(arrow);
                        f.render_widget(para, Rect::new(mid.saturating_sub(scroll_x), sy, 1, 1));
                    }
                }
            }
        }
    }
}
