use std::collections::{HashMap, HashSet};
use crate::node::{NodeID, NodeMap};

pub mod roles;
pub mod fallback;
pub mod snapshot;

pub use roles::recalculate_roles;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Coords {
    pub x: i16,
    pub y: i16,
}

pub const SIBLING_SPACING_X: i16 = 3;
pub const MIN_NODE_GAP: i16 = 3;
/// Vertical spacing between parent and child nodes.
/// Increased for better readability in auto-arrange mode.
pub const CHILD_SPACING_Y: i16 = 2;
pub const FREE_GRID_COLUMNS: usize = 4;
pub const GEMX_HEADER_HEIGHT: i16 = 2;
pub const MAX_DEPTH: usize = 32;
pub const BASE_SPACING_X: i16 = 20;
pub const BASE_SPACING_Y: i16 = 5;
pub const SNAP_GRID_X: i16 = 4;
pub const SNAP_GRID_Y: i16 = 2;
pub const RESERVED_ZONE_W: i16 = 6;
pub const RESERVED_ZONE_H: i16 = 6;

/// Estimate the width and height of a label in grid units.
///
/// Returns `(width, height)` where width is the longest line length plus a
/// padding of 2 characters and height is the number of lines.
pub fn label_bounds(label: &str) -> (i16, i16) {
    let mut width = 0i16;
    let mut height = 0i16;
    for line in label.lines() {
        height += 1;
        width = width.max(line.chars().count() as i16);
    }
    if height == 0 {
        height = 1;
    }
    (width + 2, height)
}

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
            let (label_w, _) = label_bounds(&node.label);
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
            tracing::debug!(
                "wrap pack column x={} size={:?} max_width={}",
                self.x,
                size,
                self.max_width
            );
            self.x = 0;
            self.y += self.max_height + row_padding;
            self.max_height = 0;
        }
        let anchor = (self.x, self.y);
        tracing::debug!("pack insert size {:?} at {:?}", size, anchor);
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
    term_height: i16,
    auto_arrange: bool,
) -> (HashMap<NodeID, Coords>, HashMap<NodeID, LayoutRole>) {
    tracing::debug!("[LAYOUT] layout_nodes root {} auto_arrange {}", root_id, auto_arrange);
    if nodes.is_empty() {
        tracing::debug!("layout_nodes: skip -- empty node map");
        return (HashMap::new(), HashMap::new());
    }

    let Some(root_node) = nodes.get(&root_id) else {
        tracing::debug!("layout_nodes: skip -- root {} missing", root_id);
        return (HashMap::new(), HashMap::new());
    };

    if let Some(pid) = root_node.parent {
        if !nodes.contains_key(&pid) {
            tracing::debug!(
                "layout_nodes: skip -- root {} has invalid parent {}",
                root_id,
                pid
            );
            return (HashMap::new(), HashMap::new());
        }
    }

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
        term_height,
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

    for pos in coords.values_mut() {
        pos.x -= pos.x % SNAP_GRID_X;
        pos.y -= pos.y % SNAP_GRID_Y;
    }

    if auto_arrange {
        use std::collections::HashSet;
        let mut used: HashSet<(i16, i16)> = HashSet::new();
        for (id, pos) in coords.iter_mut() {
            let (bw, bh) = nodes
                .get(id)
                .map(|n| label_bounds(&n.label))
                .unwrap_or((2, 1));
            loop {
                if pos.x == 0 {
                    pos.x += SNAP_GRID_X;
                }
                if pos.y == 0 {
                    pos.y += SNAP_GRID_Y;
                }
                let mut collision = false;
                for dx in 0..bw {
                    for dy in 0..bh {
                        if used.contains(&(pos.x + dx, pos.y + dy)) {
                            collision = true;
                            break;
                        }
                    }
                    if collision {
                        break;
                    }
                }
                if !collision {
                    for dx in 0..bw {
                        for dy in 0..bh {
                            used.insert((pos.x + dx, pos.y + dy));
                        }
                    }
                    break;
                }
                pos.x += SNAP_GRID_X;
                pos.y += SNAP_GRID_Y;
                pos.y = pos.y.max(GEMX_HEADER_HEIGHT);
                pos.x -= pos.x % SNAP_GRID_X;
                pos.y -= pos.y % SNAP_GRID_Y;
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
    term_height: i16,
    out: &mut HashMap<NodeID, Coords>,
    roles: &mut HashMap<NodeID, LayoutRole>,
    auto_arrange: bool,
    visited: &mut HashSet<NodeID>,
    depth: usize,
) -> (i16, i16, i16) {
    if !visited.insert(node_id) || depth > MAX_DEPTH {
        tracing::error!("[LAYOUT] recursion clamp at node {} depth {}", node_id, depth);
        crate::log_warn!(
            "⚠ Recursion halted: Node {} (depth {})",
            node_id,
            depth
        );
        return (y, x, x);
    }

    let node = match nodes.get(&node_id) {
        Some(n) => n,
        None => return (y, x, x),
    };

    let role = if depth == 0 {
        // Always treat the initial node as a root regardless of mode so that
        // fallback-promoted nodes remain visible.
        LayoutRole::Root
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

    let (label_width, label_height) = label_bounds(&node.label);
    out.insert(node_id, Coords { x, y });

    if node.collapsed || node.children.is_empty() {
        return (y + label_height - 1, x, x + label_width);
    }

    let child_y = y + label_height + (CHILD_SPACING_Y - 1);

    let mut max_y = y;
    let mut min_x_span = x;
    let mut max_x_span = x + label_width;

    let mut col_x = x;
    let mut col_y = child_y;
    let mut column_width = 0;
    for child_id in node.children.iter() {
        let child_h = subtree_depth(nodes, *child_id) * CHILD_SPACING_Y + 1;
        let subtree_w = subtree_span(nodes, *child_id);
        let label_w_child = nodes
            .get(child_id)
            .map(|c| label_bounds(&c.label).0)
            .unwrap_or(2);
        let child_w = subtree_w.max(label_w_child + MIN_NODE_GAP);
        if col_y + child_h > term_height {
            tracing::debug!("wrap column for node {}", child_id);
            col_y = child_y;
            col_x += column_width + SIBLING_SPACING_X;
            column_width = 0;
        }

        if col_y + child_h > term_height {
            tracing::debug!("overflow node {} beyond height {}", child_id, term_height);
        }

        let (cy, mi, ma) = layout_recursive_safe(
            nodes,
            *child_id,
            col_x,
            col_y,
            _term_width,
            term_height,
            out,
            roles,
            auto_arrange,
            visited,
            depth + 1,
        );
        max_y = max_y.max(cy);
        min_x_span = min_x_span.min(mi);
        max_x_span = max_x_span.max(ma);

        col_y = cy + CHILD_SPACING_Y;
        column_width = column_width.max(child_w);
    }

    (max_y, min_x_span.min(x), max_x_span.max(x + label_width))
}

#[allow(dead_code)]
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

/// Returns true if the position falls within the protected top-right zone.
pub fn in_reserved_zone(x: i16, y: i16, term_width: i16) -> bool {
    x >= term_width - RESERVED_ZONE_W && y < RESERVED_ZONE_H
}

/// Shift coordinates left if they collide with the protected zone.
pub fn avoid_reserved_zone(pos: &mut Coords, term_width: i16) {
    if in_reserved_zone(pos.x, pos.y, term_width) {
        pos.x = (term_width - RESERVED_ZONE_W - SNAP_GRID_X).max(0);
    }
}

/// Apply [`avoid_reserved_zone`] to every entry in the map.
pub fn avoid_reserved_zone_map(map: &mut HashMap<NodeID, Coords>, term_width: i16) {
    for pos in map.values_mut() {
        avoid_reserved_zone(pos, term_width);
    }
}

/// Center the viewport on the given node id.
///
/// Works for both auto-arrange and manual layout modes. When zoom is locked by
/// the user, scrolling only occurs if the node would otherwise be off-screen.
pub fn center_on_node(state: &mut crate::state::AppState, node_id: NodeID) {
    use std::collections::HashMap;

    if !state.nodes.contains_key(&node_id) {
        return;
    }

    let (tw, th) = if std::env::var("PRISMX_TEST").is_ok() {
        (80, 20)
    } else {
        crossterm::terminal::size().unwrap_or((80, 20))
    };
    let zoom = state.zoom_scale;
    let (bsx, bsy) = spacing_for_zoom(state.zoom_scale);

    let (nx, ny) = if state.auto_arrange {
        let roots = if let Some(root) = state.drawing_root {
            vec![root]
        } else {
            state.root_nodes.clone()
        };
        let mut layout = HashMap::new();
        let mut pack = PackRegion::new(tw as i16 - RESERVED_ZONE_W, GEMX_HEADER_HEIGHT);
        for &rid in &roots {
            let w = subtree_span(&state.nodes, rid);
            let h = subtree_depth(&state.nodes, rid) * CHILD_SPACING_Y + 1;
            let (ox, oy) = pack.insert((w, h));
            let (mut l, _) = layout_nodes(
                &state.nodes,
                rid,
                oy,
                tw as i16,
                th as i16,
                true,
            );
            for pos in l.values_mut() {
                pos.x += ox;
            }
            layout.extend(l);
        }
        avoid_reserved_zone_map(&mut layout, tw as i16);
        layout.get(&node_id).map(|c| (c.x, c.y)).unwrap_or_else(|| {
            state
                .nodes
                .get(&node_id)
                .map(|n| (n.x, n.y))
                .unwrap_or((0, 0))
        })
    } else {
        state
            .nodes
            .get(&node_id)
            .map(|n| (n.x, n.y))
            .unwrap_or((0, 0))
    };

    let draw_x = ((nx as f32 - state.scroll_x as f32) * bsx as f32 * zoom).round();
    let draw_y = ((ny as f32 - state.scroll_y as f32) * bsy as f32 * zoom).round();

    if state.zoom_locked_by_user
        && draw_x >= 0.0
        && draw_x < tw as f32
        && draw_y >= 0.0
        && draw_y < th as f32
    {
        return;
    }

    state.scroll_x = (nx as f32 - (tw as f32 / 2.0) / (bsx as f32 * zoom)).round() as i16;
    state.scroll_y = (ny as f32 - (th as f32 / 2.0) / (bsy as f32 * zoom)).round() as i16;
    clamp_scroll(state);
}
