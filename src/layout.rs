use std::collections::{HashMap, HashSet};
use crate::node::{NodeID, NodeMap};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Coords {
    pub x: i16,
    pub y: i16,
}

pub const SIBLING_SPACING_X: i16 = 3;
pub const MIN_NODE_GAP: i16 = 3;
pub const CHILD_SPACING_Y: i16 = 1;
pub const FREE_GRID_COLUMNS: usize = 4;
pub const GEMX_HEADER_HEIGHT: i16 = 2;
pub const MAX_LAYOUT_DEPTH: usize = 50;
pub const BASE_SPACING_X: i16 = 20;
pub const BASE_SPACING_Y: i16 = 5;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LayoutRole {
    Root,
    Child,
    Free,
    Orphan,
    Anchor,
    Ghost,
    Portal,
}

pub fn layout_nodes(
    nodes: &NodeMap,
    root_id: NodeID,
    start_y: i16,
    term_width: i16,
    auto_arrange: bool,
) -> (HashMap<NodeID, Coords>, HashMap<NodeID, LayoutRole>) {
    let start_y = start_y.max(GEMX_HEADER_HEIGHT + 1);
    let start_x = term_width / 2;
    let mut coords = HashMap::new();
    let mut roles = HashMap::new();
    let mut visited = HashSet::new();
    let _ = layout_recursive_safe(
        nodes,
        root_id,
        start_x,
        start_y,
        term_width,
        &mut coords,
        &mut roles,
        auto_arrange,
        &mut visited,
        0,
    );
    (coords, roles)
}

fn layout_recursive_safe(
    nodes: &NodeMap,
    node_id: NodeID,
    x: i16,
    y: i16,
    _term_width: i16,
    out: &mut HashMap<NodeID, Coords>,
    roles: &mut HashMap<NodeID, LayoutRole>,
    auto_arrange: bool,
    visited: &mut HashSet<NodeID>,
    depth: usize,
) -> (i16, i16, i16) {
    if !visited.insert(node_id) || depth > MAX_LAYOUT_DEPTH {
        return (y, x, x);
    }

    let node = match nodes.get(&node_id) {
        Some(n) => n,
        None => return (y, x, x),
    };

    let role = if depth == 0 {
        if auto_arrange {
            LayoutRole::Root
        } else {
            LayoutRole::Free
        }
    } else {
        match node.parent {
            Some(pid) => {
                if nodes.get(&pid).is_some() {
                    LayoutRole::Child
                } else {
                    LayoutRole::Orphan
                }
            }
            None => LayoutRole::Orphan,
        }
    };
    roles.insert(node_id, role);

    if node.collapsed || node.children.is_empty() {
        out.insert(node_id, Coords { x, y });
        return (y, x, x + node.label.len() as i16 + 2);
    }

    let child_y = y + CHILD_SPACING_Y;
    let mut spans = Vec::new();
    let mut total_width = 0;
    let mut max_y = y;
    let mut min_x_span = x;
    let mut max_x_span = x + node.label.len() as i16 + 2;

    for child_id in &node.children {
        let (branch_max_y, branch_min_x, branch_max_x) = layout_recursive_safe(
            nodes,
            *child_id,
            x,
            child_y,
            _term_width,
            out,
            roles,
            auto_arrange,
            visited,
            depth + 1,
        );
        let subtree_width = branch_max_x - branch_min_x;
        let label_width = nodes.get(child_id).map(|c| c.label.len() as i16).unwrap_or(0);
        let child_span = subtree_width.max(label_width) + MIN_NODE_GAP;
        spans.push((*child_id, child_span, branch_min_x, branch_max_x));
        total_width += child_span;
        max_y = max_y.max(branch_max_y);
    }

    let mut cursor_x = x - total_width / 2;
    for (child_id, child_span, branch_min_x, branch_max_x) in spans {
        let current_x = out.get(&child_id).map(|c| c.x).unwrap_or(x);
        let dx = cursor_x - current_x;
        shift_subtree(child_id, dx, out, nodes);
        min_x_span = min_x_span.min(branch_min_x + dx);
        max_x_span = max_x_span.max(branch_max_x + dx);
        cursor_x += child_span;
    }

    let new_x = (min_x_span + max_x_span) / 2;
    out.insert(node_id, Coords { x: new_x, y });
    min_x_span = min_x_span.min(new_x);
    max_x_span = max_x_span.max(new_x + node.label.len() as i16 + 2);

    (max_y, min_x_span, max_x_span)
}

fn shift_subtree(id: NodeID, dx: i16, out: &mut HashMap<NodeID, Coords>, nodes: &NodeMap) {
    if dx == 0 {
        return;
    }
    if let Some(pos) = out.get_mut(&id) {
        pos.x += dx;
    }
    if let Some(node) = nodes.get(&id) {
        if !node.collapsed {
            for child in &node.children {
                shift_subtree(*child, dx, out, nodes);
            }
        }
    }
}

/// Recenter scroll offsets so the given node remains the anchor after a zoom change.
pub fn zoom_to_anchor(state: &mut crate::state::AppState, node_id: NodeID) {
    if let Some(node) = state.nodes.get(&node_id) {
        let (tw, th) = crossterm::terminal::size().unwrap_or((80, 20));
        let zoom = state.zoom_scale;
        let anchor_x = node.x as f32 * crate::layout::BASE_SPACING_X as f32 * zoom;
        let anchor_y = node.y as f32 * crate::layout::BASE_SPACING_Y as f32 * zoom;
        state.scroll_x = (anchor_x - tw as f32 / 2.0).round() as i16;
        state.scroll_y = (anchor_y - th as f32 / 2.0).round() as i16;
        clamp_scroll(state);
    }
}

/// Ensure scroll offsets never go below zero.
pub fn clamp_scroll(state: &mut crate::state::AppState) {
    if state.scroll_x < 0 {
        state.scroll_x = 0;
    }
    if state.scroll_y < 0 {
        state.scroll_y = 0;
    }
}
