use ratatui::{prelude::*, widgets::Paragraph};
use crate::node::{NodeID, NodeMap};
use crate::state::AppState;
use crate::layout::engine::{center_x, layout_vertical};
use super::layout::clamp_child_spacing;
use crate::ui::lines::{
    draw_vertical_fade,
    draw_horizontal_shimmer,
    draw_ghost_line,
    draw_anchor_trail,
};
use crate::theme::beam_color::{parent_line_color, sibling_line_color};
use crate::beam_color::BeamColor;
use crate::ui::beamx::{BeamXMode, BeamXStyle, InsertCursorKind, render_insert_cursor};
use crate::theme::icons::{ROOT_NODE, CHILD_NODE, SIBLING_NODE};

/// Render a simple mindmap using the vertical layout engine.
pub fn render<B: Backend>(
    f: &mut Frame<B>,
    area: Rect,
    nodes: &mut NodeMap,
    roots: &[NodeID],
    state: &AppState,
    debug: bool,
) {
    let spacing_y = clamp_child_spacing(state, roots, area.height as i16);
    for &root in roots {
        layout_vertical(nodes, root, spacing_y);
    }

    let tick = (std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis()
        / 300) as u64;
    let theme = BeamColor::Prism;
    let p_color = parent_line_color(theme);
    let s_color = sibling_line_color(theme);
    let mut cursor_style = BeamXStyle::from(BeamXMode::Zen);
    cursor_style.border_color = p_color;
    cursor_style.status_color = s_color;

    // Determine scroll offset if content exceeds available height
    let max_y = nodes.values().map(|n| n.y).max().unwrap_or(0);
    let mut scroll = 0i16;
    if max_y >= area.height as i16 {
        scroll = max_y - area.height as i16 + 1;
    }

    if debug {
        let ox = area.x as i16;
        let oy = area.y as i16;

        for node in nodes.values() {
            if node.children.is_empty() {
                continue;
            }

            let px = center_x(nodes, node.id);
            let beam_y = node.y + (spacing_y - 1).max(1);

            // vertical beam from parent to sibling bar
            draw_vertical_fade(
                f,
                (px + ox, node.y + 1 + oy - scroll),
                (px + ox, beam_y + oy - scroll),
                tick,
                p_color,
            );

            // collect child centers
            let mut child_centers = Vec::new();
            for cid in &node.children {
                if nodes.contains_key(cid) {
                    child_centers.push((cid, center_x(nodes, *cid)));
                }
            }
            if child_centers.is_empty() {
                continue;
            }
            let min_x = child_centers.iter().map(|c| c.1).min().unwrap();
            let max_x = child_centers.iter().map(|c| c.1).max().unwrap();

            // horizontal connector across siblings
            draw_horizontal_shimmer(
                f,
                (min_x + ox, beam_y + oy - scroll),
                (max_x + ox, beam_y + oy - scroll),
                tick,
                s_color,
            );

            // vertical drop to each child
            for (cid, cx) in child_centers {
                if let Some(child) = nodes.get(cid) {
                    let start = (cx + ox, beam_y + oy - scroll);
                    let end = (cx + ox, child.y + oy - scroll);
                    let is_ghost = child.label == "New Child" || child.label == "New Sibling";
                    if is_ghost {
                        draw_ghost_line(f, start, end, tick, p_color);
                    } else {
                        draw_vertical_fade(f, start, end, tick, p_color);
                    }
                }
            }
        }
    }

    // Preview reparent link if dragging over a potential parent
    if let (Some(src), Some(tgt)) = (state.dragging, state.drag_hover_target) {
        if let (Some(s), Some(t)) = (nodes.get(&src), nodes.get(&tgt)) {
            let sx = area.x as i16 + center_x(nodes, src);
            let sy = area.y as i16 + s.y - scroll;
            let tx = area.x as i16 + center_x(nodes, tgt);
            let ty = area.y as i16 + t.y - scroll;
            draw_anchor_trail(f, (sx, sy), (tx, ty), tick, p_color);
        }
    }

    // Draw nodes
    for node in nodes.values() {
        let x = area.x as i16 + node.x;
        let y = area.y as i16 + node.y - scroll;
        if x >= 0 && y >= 0 && x < area.right() as i16 && y < area.bottom() as i16 {
            let mut text = node.label.clone();
            if state.hierarchy_icons {
                let icon = if node.parent.is_none() {
                    ROOT_NODE
                } else {
                    let parent_id = node.parent.unwrap();
                    if nodes.get(&parent_id)
                        .and_then(|p| p.children.first().copied()) == Some(node.id)
                    {
                        CHILD_NODE
                    } else {
                        SIBLING_NODE
                    }
                };
                text = format!("{} {}", icon, text);
            }
            let rect = Rect::new(x as u16, y as u16, text.len() as u16, 1);
            f.render_widget(Paragraph::new(text.clone()), rect);

            if node.label == "New Child" || node.label == "New Sibling" {
                let kind = if node.label == "New Child" {
                    InsertCursorKind::Child
                } else {
                    InsertCursorKind::Sibling
                };
                let cx = rect.x + rect.width;
                let cy = rect.y;
                render_insert_cursor(f, (cx, cy), tick, kind, &cursor_style);
            }
        }
    }
}
