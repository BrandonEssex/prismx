use crate::state::{AppState, LayoutStyle};
use crate::node::{NodeID, NodeMap};
use super::viewport;
pub use crate::layout::engine::{sibling_offset, next_free_horizontal, next_free_vertical};
use crossterm::terminal;
use ratatui::prelude::Rect;
use crate::layout::GEMX_HEADER_HEIGHT;

/// Minimum node count before the logical grid is allowed to expand.
const GRID_EXPANSION_THRESHOLD: usize = 50;

/// Ensure inserted nodes don't overlap existing ones.
fn repair_insert(state: &mut AppState, node_id: NodeID) {
    use crate::layout::{SIBLING_SPACING_X, CHILD_SPACING_Y};

    let prev = state.selection_trail.back().map(|(id, _)| *id);
    let parent = state.nodes.get(&node_id).and_then(|n| n.parent);

    if let Some(prev_id) = prev {
        if Some(prev_id) == parent {
            if let Some(p) = state.nodes.get(&parent.unwrap()).cloned() {
                let base_y = p.y + CHILD_SPACING_Y;
                let new_y = next_free_vertical(&state.nodes, p.x, base_y, CHILD_SPACING_Y, node_id);
                if let Some(n) = state.nodes.get_mut(&node_id) {
                    n.x = p.x;
                    n.y = new_y;
                }
            }
        } else if state.nodes.get(&prev_id).and_then(|n| n.parent) == parent {
            if let Some(sel) = state.nodes.get(&prev_id).cloned() {
                let base_x = sel.x + SIBLING_SPACING_X;
                let new_x = next_free_horizontal(&state.nodes, base_x, sel.y, SIBLING_SPACING_X, node_id);
                if let Some(n) = state.nodes.get_mut(&node_id) {
                    n.x = new_x;
                    n.y = sel.y;
                }
            }
        }
    }

    if parent.is_some() && state.nodes.get(&node_id).map(|n| (n.x, n.y)) == Some((0, 0)) {
        let x = next_free_horizontal(&state.nodes, SIBLING_SPACING_X, 0, SIBLING_SPACING_X, node_id);
        let y = CHILD_SPACING_Y;
        if let Some(n) = state.nodes.get_mut(&node_id) {
            n.x = x;
            n.y = y;
        }
    }

    if let Some(n) = state.nodes.get(&node_id) {
        println!("LAYOUT_OK: node={} x={} y={}", n.id, n.x, n.y);
    }
}
/// Ensure the newly inserted node remains visible by centering on it.
pub fn focus_new_node(state: &mut AppState, node_id: NodeID) {
    repair_insert(state, node_id);
    // Auto-arrange jumps immediately to avoid excessive animation when the
    // entire layout may shift, otherwise scroll smoothly toward the target.
    let jump = state.auto_arrange;
    viewport::recenter_on_node(state, node_id, jump);

    // After inserting a node we want the immediate layout to remain coherent.
    // Realign siblings around the parent to avoid overlap and maintain
    // consistent spacing before the full layout pass.
    if let Some(parent_id) = state.nodes.get(&node_id).and_then(|n| n.parent) {
        let (_, th) = terminal::size().unwrap_or((80, 20));
        let spacing = clamp_child_spacing(state, &[parent_id], th as i16);
        crate::layout::engine::reflow_siblings(&mut state.nodes, parent_id, spacing);
    }

    // Reflow root clusters so the focused branch remains stationary while
    // peers shift outward if needed.
    reflow_around_focus(state);
}

/// Attempt to focus the most recently visible node if `node_id` is missing.
pub fn focus_or_recent(state: &mut AppState, node_id: Option<NodeID>) {
    if let Some(id) = node_id {
        state.set_selected(Some(id));
        focus_new_node(state, id);
        return;
    }

    let mut target = None;
    for (nid, _) in state.selection_trail.iter().rev() {
        if state.nodes.contains_key(nid) && viewport::node_visible(state, *nid) {
            target = Some(*nid);
            break;
        }
    }
    if let Some(id) = target {
        state.set_selected(Some(id));
        focus_new_node(state, id);
    }
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

    // Start with the current viewport as the minimum bounding box. Only allow
    // the grid to expand when the node count is large enough to justify it.
    let mut max_x = area.right() as i16 - 2;
    let mut max_y = area.bottom() as i16 - 2;
    if nodes.len() >= GRID_EXPANSION_THRESHOLD {
        for n in nodes.values() {
            if n.x >= max_x - 1 {
                max_x = n.x + crate::layout::SIBLING_SPACING_X;
            }
            if n.y >= max_y - 1 {
                max_y = n.y + crate::layout::CHILD_SPACING_Y;
            }
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

    // Scale spacing relative to zoom level so nodes remain visually connected
    // when zooming between 0.5x and 1.5x.
    let zoom = state.zoom_scale.clamp(0.5, 1.5);
    spacing = (((spacing as f32) * zoom) - 1.0).round() as i16;

    // Apply style bias
    spacing = match state.layout_style {
        LayoutStyle::Compact => spacing.saturating_sub(1),
        LayoutStyle::Relaxed => spacing + 1,
    };
    if spacing < MIN_CHILD_SPACING_Y {
        spacing = MIN_CHILD_SPACING_Y;
    }
    if spacing > CHILD_SPACING_Y * 2 {
        spacing = CHILD_SPACING_Y * 2;
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
///
/// When a tree contains extremely deep branches the layout engine begins to
/// stagger child nodes horizontally (see [`DEEP_BRANCH_THRESHOLD`]). Each root
/// cluster therefore requires additional horizontal padding to account for this
/// diagonal fan-out so connectors remain clean and nodes don't collide.
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
