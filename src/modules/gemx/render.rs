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
    draw_line,
    draw_arrow,
};
use crate::theme::beam_color::{parent_line_color, sibling_line_color};
use crate::beam_color::BeamColor;
use crate::ui::beamx::{BeamXMode, BeamXStyle, InsertCursorKind, render_insert_cursor};
use crate::theme::icons::{ROOT_NODE, CHILD_NODE, SIBLING_NODE};
use crate::theme::characters::{DOWN_ARROW, RIGHT_ARROW};

fn compressed_label(label: &str, zoom: f32) -> String {
    if zoom > 0.5 {
        return label.to_string();
    }

    let mut base = label.split(|c| c == '#' || c == '@' || c == '[').next().unwrap_or(label).trim().to_string();
    if base.len() > 10 {
        base.truncate(10);
        base.push('…');
    }
    base
}

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

    let zoom = state.zoom_scale as f32;

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

    let scale_x = |val: i16| -> i16 { area.x as i16 + ((val as f32) * zoom).round() as i16 };
    let scale_y = |val: i16| -> i16 { area.y as i16 + (((val - scroll) as f32) * zoom).round() as i16 };

    if debug {
        for node in nodes.values() {
            if node.children.is_empty() {
                continue;
            }

            let px = scale_x(center_x(nodes, node.id));
            let beam_y = scale_y(node.y + (spacing_y - 1).max(1));

            // vertical beam from parent to sibling bar
            let parent_start = (px, scale_y(node.y + 1));
            let parent_end = (px, beam_y);
            if state.beam_shimmer {
                draw_vertical_fade(f, parent_start, parent_end, tick, p_color);
            } else {
                draw_line(f, parent_start, parent_end);
            }
            draw_arrow(
                f,
                parent_end,
                tick,
                p_color,
                DOWN_ARROW,
                state.beam_shimmer,
            );

            // collect child centers
            let mut child_centers = Vec::new();
            for cid in &node.children {
                if nodes.contains_key(cid) {
                    child_centers.push((cid, scale_x(center_x(nodes, *cid))));
                }
            }
            if child_centers.is_empty() {
                continue;
            }
            let min_x = child_centers.iter().map(|c| c.1).min().unwrap();
            let max_x = child_centers.iter().map(|c| c.1).max().unwrap();

            // horizontal connector across siblings
            if state.beam_shimmer {
                draw_horizontal_shimmer(
                    f,
                    (min_x, beam_y),
                    (max_x, beam_y),
                    tick,
                    s_color,
                );
            } else {
                draw_line(
                    f,
                    (min_x, beam_y),
                    (max_x, beam_y),
                );
            }

            // arrow across siblings
            let mid = ((min_x + max_x) / 2) as i16;
            draw_arrow(
                f,
                (mid, beam_y),
                tick,
                s_color,
                RIGHT_ARROW,
                state.beam_shimmer,
            );

            // vertical drop to each child
            for (cid, cx) in child_centers {
                if let Some(child) = nodes.get(cid) {
                    let start = (cx, beam_y);
                    let end = (cx, scale_y(child.y));
                    let is_ghost = child.label.starts_with("node ");
                    if state.beam_shimmer {
                        if is_ghost {
                            draw_ghost_line(f, start, end, tick, p_color);
                        } else {
                            draw_vertical_fade(f, start, end, tick, p_color);
                        }
                    } else {
                        draw_line(f, start, end);
                    }
                    draw_arrow(
                        f,
                        end,
                        tick,
                        p_color,
                        DOWN_ARROW,
                        state.beam_shimmer,
                    );
                }
            }
        }
    }

    // Preview reparent link if dragging over a potential parent
    if let (Some(src), Some(tgt)) = (state.dragging, state.drag_hover_target) {
        if let (Some(s), Some(t)) = (nodes.get(&src), nodes.get(&tgt)) {
            let ax = scale_x(center_x(nodes, src));
            let ay = scale_y(s.y);
            let bx = scale_x(center_x(nodes, tgt));
            let by = scale_y(t.y);
            draw_anchor_trail(f, (ax, ay), (bx, by), tick, p_color);
        }
    }

    // Draw nodes
    for node in nodes.values() {
        let x = scale_x(node.x);
        let y = scale_y(node.y);
        if x >= 0 && y >= 0 && x < area.right() as i16 && y < area.bottom() as i16 {
            let mut text = compressed_label(&node.label, zoom);
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
            let width = ((text.len() as f32) * zoom).ceil().max(1.0) as u16;
            let display = if zoom <= 0.5 { text } else { node.label.clone() };
            let rect = Rect::new(x as u16, y as u16, width, 1);
            f.render_widget(Paragraph::new(display.clone()), rect);

            if node.label.starts_with("node ") {
                let kind = InsertCursorKind::Sibling;
                let cx = rect.x + rect.width;
                let cy = rect.y;
                render_insert_cursor(f, (cx, cy), tick, kind, &cursor_style);
            }
        }
    }
}
