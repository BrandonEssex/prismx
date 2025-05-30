use crate::state::AppState;
use crate::node::{NodeID, NodeMap};
use super::viewport;
pub use crate::layout::engine::sibling_offset;
use crossterm::terminal;
use ratatui::prelude::Rect;
use crate::layout::{
    GEMX_HEADER_HEIGHT,
    SIBLING_SPACING_X,
    CHILD_SPACING_Y,
};

/// Ensure the newly inserted node remains visible by centering on it.
pub fn focus_new_node(state: &mut AppState, node_id: NodeID) {
    viewport::ensure_visible(state, node_id);
    // After inserting a node we want the immediate layout to remain coherent.
    // Reflow sibling groups around the focused branch so the new node does not
    // cause the entire tree to shift unpredictably.
    reflow_around_focus(state);
}

/// Follow the currently selected node each frame.
pub fn follow_focus_node(state: &mut AppState) {
    viewport::follow_focus(state);
}

/// Reposition root branches so the currently focused branch remains stationary
/// while its siblings shift outward if necessary. This keeps horizontal
/// alignment stable when inserting or expanding nodes.
pub fn reflow_around_focus(state: &mut AppState) {
    let focus = focused_root(state);
    preserve_focused_branch(&mut state.nodes, &state.root_nodes, focus);
}

/// Clamp scroll offsets relative to zoom level to avoid jumpiness.
pub fn clamp_zoom_scroll(state: &mut AppState) {
    let base_limit = (1000.0 * state.zoom_scale) as i16;

    // Expand scroll limits based on farthest node positions so large mindmaps
    // remain reachable without reflowing existing layout.
    let mut max_x = 0i16;
    let mut max_y = 0i16;
    for n in state.nodes.values() {
        max_x = max_x.max(n.x);
        max_y = max_y.max(n.y);
    }
    let limit_x = base_limit.max(max_x + 10);
    let limit_y = base_limit.max(max_y + 10);

    // Give the user some buffer space when fully zoomed out so nodes aren't
    // forced against the viewport edges. Padding increases as zoom decreases
    // and is more aggressive when zoomed far out.
    let pad_right = if state.zoom_scale <= 0.3 {
        60
    } else if state.zoom_scale <= 0.5 {
        40
    } else {
        20
    };
    let pad_vert = if state.zoom_scale <= 0.3 {
        30
    } else if state.zoom_scale <= 0.5 {
        20
    } else {
        10
    };

    state.scroll_x = state.scroll_x.clamp(0, limit_x + pad_right);
    state.scroll_y = state.scroll_y.clamp(0, limit_y + pad_vert);
    state.scroll_target_x = state.scroll_target_x.clamp(0, limit_x + pad_right);
    state.scroll_target_y = state.scroll_target_y.clamp(0, limit_y + pad_vert);
}

/// Ensure all nodes remain within the visible viewport area.
pub fn enforce_viewport_bounds(nodes: &mut NodeMap, area: Rect) {
    let min_x = area.x as i16 + 1;
    let min_y = GEMX_HEADER_HEIGHT.max(area.y as i16 + 1);

    // Start with the current viewport as the minimum bounding box then grow it
    // if any nodes fall outside the visible region. This allows the logical
    // grid to expand as the mindmap grows rather than clamping coordinates.
    let mut max_x = area.right() as i16 - 2;
    let mut max_y = area.bottom() as i16 - 2;
    for n in nodes.values() {
        if n.x >= max_x - 1 {
            max_x = n.x + crate::layout::SIBLING_SPACING_X;
        }
        if n.y >= max_y - 1 {
            max_y = n.y + crate::layout::CHILD_SPACING_Y;
        }
    }

    for node in nodes.values_mut() {
        node.x = node.x.clamp(min_x, max_x);
        node.y = node.y.clamp(min_y, max_y);
    }
}

/// Determine dynamic child spacing based on total depth.
pub fn clamp_child_spacing(state: &AppState, roots: &[NodeID], max_h: i16) -> i16 {
    use crate::layout::{CHILD_SPACING_Y, MIN_CHILD_SPACING_Y, subtree_depth};

    let mut depth = 1i16;
    for r in roots {
        depth = depth.max(subtree_depth(&state.nodes, *r));
    }
    let required = depth * CHILD_SPACING_Y;
    let mut spacing = if max_h > 0 && required > max_h {
        let ratio = max_h as f32 / required as f32;
        ((CHILD_SPACING_Y as f32 * ratio).floor() as i16).max(MIN_CHILD_SPACING_Y)
    } else {
        CHILD_SPACING_Y
    };

    if (state.zoom_scale - 1.0).abs() < f32::EPSILON {
        spacing = (spacing - 1).max(MIN_CHILD_SPACING_Y);
    } else if state.zoom_scale < 1.0 {
        spacing = ((spacing as f32) * state.zoom_scale)
            .floor()
            .max(MIN_CHILD_SPACING_Y as f32) as i16;
    } else if state.zoom_scale > 1.0 {
        spacing = ((spacing as f32) * state.zoom_scale).ceil() as i16;
    }

    spacing
}

/// Return the root ancestor for the currently selected node.
pub fn focused_root(state: &AppState) -> Option<NodeID> {
    let mut current = state.selected?;
    while let Some(parent) = state.nodes.get(&current).and_then(|n| n.parent) {
        current = parent;
    }
    Some(current)
}

fn subtree_bounds(nodes: &crate::node::NodeMap, id: NodeID) -> (i16, i16) {
    use crate::layout::{label_bounds, subtree_span};
    let span = subtree_span(nodes, id);
    let (w, _) = nodes
        .get(&id)
        .map(|n| label_bounds(&n.label))
        .unwrap_or((2, 1));
    let left = nodes.get(&id).map(|n| n.x - (span - w) / 2).unwrap_or(0);
    (left, left + span)
}

fn shift_subtree(nodes: &mut crate::node::NodeMap, id: NodeID, dx: i16) {
    fn walk(nodes: &mut crate::node::NodeMap, nid: NodeID, dx: i16) {
        if let Some(node) = nodes.get_mut(&nid) {
            node.x += dx;
            if !node.collapsed {
                let children = node.children.clone();
                for c in children {
                    walk(nodes, c, dx);
                }
            }
        }
    }
    if dx != 0 {
        walk(nodes, id, dx);
    }
}

/// Calculate dynamic spacing between root clusters based on available width.
fn root_spacing(nodes: &crate::node::NodeMap, roots: &[NodeID]) -> i16 {
    use crate::layout::{subtree_span, subtree_depth, SIBLING_SPACING_X, MIN_SIBLING_SPACING_X, RESERVED_ZONE_W};
    use crate::layout::engine::{DEEP_BRANCH_THRESHOLD, DEEP_BRANCH_STEP_X};

    let (tw, _) = terminal::size().unwrap_or((80, 20));
    let max_w = tw as i16 - RESERVED_ZONE_W;

    let mut total = 0i16;
    let mut extra = 0i16;
    for r in roots {
        total += subtree_span(nodes, *r);
        let depth = subtree_depth(nodes, *r) as usize;
        if depth > DEEP_BRANCH_THRESHOLD {
            let add = ((depth - DEEP_BRANCH_THRESHOLD) as i16) * DEEP_BRANCH_STEP_X;
            extra = extra.max(add);
        }
    }
    if roots.len() > 1 {
        total += (SIBLING_SPACING_X + extra) * (roots.len() as i16 - 1);
    }

    if max_w > 0 && total > max_w {
        let ratio = max_w as f32 / total as f32;
        let spacing = (((SIBLING_SPACING_X + extra) as f32) * ratio).floor() as i16;
        spacing.max(MIN_SIBLING_SPACING_X)
    } else {
        SIBLING_SPACING_X + extra
    }
}

/// Shift peer root trees horizontally so the focused branch remains stationary.
pub fn preserve_focused_branch(
    nodes: &mut crate::node::NodeMap,
    roots: &[NodeID],
    focus_root: Option<NodeID>,
) {
    let Some(active) = focus_root else { return };

    let spacing = root_spacing(nodes, roots);

    let mut data: Vec<(NodeID, i16, i16)> = roots
        .iter()
        .copied()
        .map(|r| {
            let (l, rgt) = subtree_bounds(nodes, r);
            (r, l, rgt)
        })
        .collect();

    data.sort_by_key(|d| d.1);
    let idx = data.iter().position(|d| d.0 == active).unwrap_or(0);

    // shift peers to the right of the active root
    let mut bound = data[idx].2 + spacing;
    for (rid, left, right) in data.iter_mut().skip(idx + 1) {
        if *left < bound {
            let delta = bound - *left;
            shift_subtree(nodes, *rid, delta);
            *left += delta;
            *right += delta;
        }
        bound = *right + spacing;
    }

    // shift peers to the left of the active root
    let mut bound_left = data[idx].1 - spacing;
    for (rid, left, right) in data[..idx].iter_mut().rev() {
        if *right > bound_left {
            let delta = *right - bound_left;
            shift_subtree(nodes, *rid, -delta);
            *left -= delta;
            *right -= delta;
        }
        bound_left = *left - spacing;
    }
}
