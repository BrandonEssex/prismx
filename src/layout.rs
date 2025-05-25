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

pub fn spacing_for_zoom(zoom: f32) -> (i16, i16) {
    if zoom < 0.7 {
        (4, 2)
    } else {
        (6, 3)
    }
}

pub fn subtree_span(nodes: &NodeMap, id: NodeID) -> i16 {
    fn walk(nodes: &NodeMap, nid: NodeID, visited: &mut HashSet<NodeID>) -> i16 {
        if !visited.insert(nid) {
            return 0;
        }
        if let Some(node) = nodes.get(&nid) {
            let label_w = node.label.len() as i16 + 2;
            if node.collapsed || node.children.is_empty() {
                return label_w;
            }
            let mut total = 0;
            for (i, c) in node.children.iter().enumerate() {
                if i > 0 {
                    total += SIBLING_SPACING_X;
                }
                total += walk(nodes, *c, visited);
            }
            label_w.max(total)
        } else {
            0
        }
    }

    walk(nodes, id, &mut HashSet::new())
}

pub fn subtree_depth(nodes: &NodeMap, id: NodeID) -> i16 {
    fn walk(nodes: &NodeMap, nid: NodeID, visited: &mut HashSet<NodeID>) -> i16 {
        if !visited.insert(nid) {
            return 0;
        }
        if let Some(node) = nodes.get(&nid) {
            if node.collapsed || node.children.is_empty() {
                return 1;
            }
            let mut max_child = 0;
            for c in &node.children {
                max_child = max_child.max(walk(nodes, *c, visited));
            }
            1 + max_child
        } else {
            1
        }
    }

    walk(nodes, id, &mut HashSet::new())
}

pub struct PackRegion {
    pub x: i16,
    pub y: i16,
    pub max_height: i16,
    pub max_width: i16,
}

impl PackRegion {
    pub fn new(max_width: i16, start_y: i16) -> Self {
        Self {
            x: 0,
            y: start_y,
            max_height: 0,
            max_width,
        }
    }

    pub fn insert(&mut self, size: (i16, i16)) -> (i16, i16) {
        let margin = SIBLING_SPACING_X * 2;
        let row_padding = CHILD_SPACING_Y * 2;
        let (w, h) = size;
        if self.x + w + margin > self.max_width {
            self.x = 0;
            self.y += self.max_height + row_padding;
            self.max_height = 0;
        }
        let anchor = (self.x, self.y);
        self.x += w + margin;
        if h > self.max_height {
            self.max_height = h;
        }
        anchor
    }
}

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
    let start_y = start_y.max(GEMX_HEADER_HEIGHT);
    let start_x = if auto_arrange { 0 } else { term_width / 2 };
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

    if let Some(min_x) = coords.values().map(|c| c.x).min() {
        if min_x < 0 {
            for pos in coords.values_mut() {
                pos.x -= min_x;
            }
        }
    }

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
                    LayoutRole::Free
                }
            }
            None => LayoutRole::Free,
        }
    };
    roles.insert(node_id, role);

    let label_width = node.label.len() as i16 + 2;
    out.insert(node_id, Coords { x, y });

    if node.collapsed || node.children.is_empty() {
        return (y, x, x + label_width);
    }

    let child_y = y + CHILD_SPACING_Y;

    let mut max_y = y;
    let mut min_x_span = x;
    let mut max_x_span = x + label_width;

    let mut offset = 0;
    for child_id in node.children.iter() {
        let child_x = x + offset;
        let (cy, mi, ma) = layout_recursive_safe(
            nodes,
            *child_id,
            child_x,
            child_y,
            _term_width,
            out,
            roles,
            auto_arrange,
            visited,
            depth + 1,
        );
        max_y = max_y.max(cy);
        min_x_span = min_x_span.min(mi);
        max_x_span = max_x_span.max(ma);
        offset += BASE_SPACING_X;
    }

    (max_y, min_x_span.min(x), max_x_span.max(x + label_width))
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
        let (bsx, bsy) = spacing_for_zoom(state.zoom_scale);
        state.scroll_x = (node.x as f32 - (tw as f32 / 2.0) / (bsx as f32 * zoom)).round() as i16;
        state.scroll_y = (node.y as f32 - (th as f32 / 2.0) / (bsy as f32 * zoom)).round() as i16;
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
