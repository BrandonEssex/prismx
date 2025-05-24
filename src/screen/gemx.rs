use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, Paragraph};
use crate::layout::{
    layout_nodes, Coords, LayoutRole, PackRegion, GEMX_HEADER_HEIGHT,
    BASE_SPACING_X, BASE_SPACING_Y, CHILD_SPACING_Y, subtree_span, subtree_depth,
};
use crate::node::{NodeID, NodeMap};
use crate::state::AppState;
use crate::beamx::{render_beam_logo, render_full_border, style_for_mode};
use std::collections::HashMap;

pub fn render_gemx<B: Backend>(f: &mut Frame<B>, area: Rect, state: &AppState) {
    let style = style_for_mode(&state.mode);
    let block = Block::default()
        .title(if state.auto_arrange { "Gemx [Auto-Arrange]" } else { "Gemx" })
        .borders(Borders::NONE);
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
    let mut node_roles = HashMap::new();
    if state.auto_arrange {
        let mut pack = PackRegion::new(area.width as i16, GEMX_HEADER_HEIGHT + 1);
        for &root_id in &roots {
            let w = subtree_span(&state.nodes, root_id);
            let h = subtree_depth(&state.nodes, root_id) * CHILD_SPACING_Y + 1;
            let (ox, oy) = pack.insert((w, h));

            let (mut layout, roles) =
                layout_nodes(&state.nodes, root_id, oy, area.width as i16, state.auto_arrange);
            for pos in layout.values_mut() {
                pos.x += ox;
            }
            drawn_at.extend(layout);
            node_roles.extend(roles);
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
            let (_, roles) =
                layout_nodes(&state.nodes, root_id, 0, area.width as i16, state.auto_arrange);
            node_roles.extend(roles);
        }
    }

    for (&node_id, &Coords { x, y }) in &drawn_at {
        let role = node_roles
            .get(&node_id)
            .copied()
            .unwrap_or(LayoutRole::Child);
        if role == LayoutRole::Ghost {
            continue;
        }
        let zoom = state.zoom_scale as f32;
        let draw_x = ((x as f32 * BASE_SPACING_X as f32 * zoom) - state.scroll_x as f32)
            .round()
            .max(0.0) as u16;
        let draw_y = ((y as f32 * BASE_SPACING_Y as f32 * zoom) - state.scroll_y as f32)
            .round()
            .max(0.0) as u16;

        if draw_x >= area.width || draw_y >= area.height {
            #[cfg(debug_assertions)]
            eprintln!("[debug] clamp node ({},{})", draw_x, draw_y);
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
        if role == LayoutRole::Free {
            label.push_str(" 🧩");
        }
        if state.link_map.get(&node_id).and_then(|v| v.first()).is_some() {
            label.push_str(" 📎");
        }

        let width = label.len().min((area.width - draw_x) as usize);

        let mut style = if is_selected {
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD | Modifier::UNDERLINED)
        } else {
            Style::default().fg(Color::White)
        };
        if !is_selected {
            match role {
                LayoutRole::Root => {
                    style = style.fg(Color::Cyan).add_modifier(Modifier::BOLD);
                }
                LayoutRole::Orphan => {
                    style = style.add_modifier(Modifier::DIM | Modifier::UNDERLINED);
                }
                _ => {}
            }
        }

        let para = Paragraph::new(label).style(style);
        f.render_widget(para, Rect::new(draw_x, draw_y, width as u16, 1));
    }

    // Draw arrows
    for (source, targets) in &state.link_map {
        for target in targets {
            if let (Some(&Coords { x: sx, y: sy }), Some(&Coords { x: tx, y: ty })) =
                (drawn_at.get(source), drawn_at.get(target))
            {
                if sy == ty {
                    let zoom = state.zoom_scale as f32;
                    let sxp = ((sx as f32 * BASE_SPACING_X as f32 * zoom) - state.scroll_x as f32).round();
                    let txp = ((tx as f32 * BASE_SPACING_X as f32 * zoom) - state.scroll_x as f32).round();
                    let syp = ((sy as f32 * BASE_SPACING_Y as f32 * zoom) - state.scroll_y as f32).round();
                    let arrow = if sx < tx { "→" } else { "←" };
                    let mid = ((sxp + txp) / 2.0).round().max(0.0) as u16;
                    let draw_sy = syp.max(0.0) as u16;
                    if mid >= area.width || draw_sy >= area.height {
                        #[cfg(debug_assertions)]
                        eprintln!("[debug] clamp arrow ({},{})", mid, draw_sy);
                        continue;
                    }
                    let para = Paragraph::new(arrow);
                    f.render_widget(para, Rect::new(mid, draw_sy, 1, 1));
                }
            }
        }
    }

    render_full_border(f, area, &style);
    render_beam_logo(f, area, &style);
}
