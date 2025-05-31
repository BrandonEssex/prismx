use super::{grid, nodes, connector};
use crate::node::{NodeID, NodeMap};
use crate::layout::{label_bounds, subtree_span, MIN_CHILD_SPACING_Y, MIN_NODE_GAP};

pub use grid::{
    spacing_for_zoom,
    center_x,
    sibling_offset,
    grid_size,
    detect_overflow,
    detect_collisions,
};

pub use nodes::{
    layout_vertical,
    compute_depths,
    DEEP_BRANCH_THRESHOLD,
    DEEP_BRANCH_STEP_X,
};

pub use connector::{beam_y, parent_line, child_line};

fn depth_offset(depth: usize) -> i16 {
    if depth > DEEP_BRANCH_THRESHOLD {
        ((depth - DEEP_BRANCH_THRESHOLD) as i16) * DEEP_BRANCH_STEP_X
    } else {
        0
    }
}

fn node_depth(nodes: &NodeMap, id: NodeID) -> usize {
    let mut depth = 0usize;
    let mut current = nodes.get(&id).and_then(|n| n.parent);
    while let Some(pid) = current {
        depth += 1;
        current = nodes.get(&pid).and_then(|n| n.parent);
        if depth > nodes.len() {
            break;
        }
    }
    depth
}

/// Reflow an existing set of siblings so they remain horizontally aligned
/// relative to their parent. Only the sibling positions are mutated.
pub fn reflow_siblings(nodes: &mut NodeMap, parent: NodeID, spacing_y: i16) {
    let Some(p) = nodes.get(&parent).cloned() else { return };
    if p.children.is_empty() || p.collapsed {
        return;
    }

    let spacing_y = spacing_y.max(MIN_CHILD_SPACING_Y);
    let base_depth = node_depth(nodes, parent);

    let total_span = subtree_span(nodes, parent);
    let (label_w, _) = label_bounds(&p.label);
    let mut child_x = p.x - (total_span - label_w) / 2;
    let child_y = p.y + spacing_y;
    let len = p.children.len();

    for (i, cid) in p.children.iter().copied().enumerate() {
        let span = subtree_span(nodes, cid);
        let label_w_child = nodes
            .get(&cid)
            .map(|c| label_bounds(&c.label).0)
            .unwrap_or(2);

        let offset = depth_offset(base_depth + 1);

        if let Some(cnode) = nodes.get_mut(&cid) {
            cnode.x = child_x + offset + (span - label_w_child) / 2;
            cnode.y = child_y;
        }

        let mut child_w = span.max(label_w_child + MIN_NODE_GAP);
        child_w += offset;
        child_x += child_w;
        if i + 1 < len {
            child_x += sibling_offset(i, len);
        }
    }
}

/// Check if any node occupies the given position, excluding `skip` if provided.
fn position_taken(nodes: &NodeMap, x: i16, y: i16, skip: Option<NodeID>) -> bool {
    for n in nodes.values() {
        if Some(n.id) != skip && n.x == x && n.y == y {
            return true;
        }
    }
    false
}

/// Find the next free horizontal slot starting from `x` at row `y`.
pub fn next_free_horizontal(nodes: &NodeMap, mut x: i16, y: i16, step: i16, skip: NodeID) -> i16 {
    while position_taken(nodes, x, y, Some(skip)) {
        x += step;
    }
    x
}

/// Find the next free vertical slot starting from `y` at column `x`.
pub fn next_free_vertical(nodes: &NodeMap, x: i16, mut y: i16, step: i16, skip: NodeID) -> i16 {
    while position_taken(nodes, x, y, Some(skip)) {
        y += step;
    }
    y
}
