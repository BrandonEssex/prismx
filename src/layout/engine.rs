use super::{connector, grid, nodes};
use crate::layout::{
    label_bounds, subtree_span, MIN_CHILD_SPACING_Y, MIN_NODE_GAP, SIBLING_SPACING_X,
    GEMX_HEADER_HEIGHT,
};
use crate::node::{Node, NodeID, NodeMap};
use crate::settings;
use ratatui::layout::Rect;

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

/// Horizontal spacing used between sibling connectors based on user settings.
pub fn lane_spacing() -> i16 {
    if settings::load_user_settings().mindmap_lanes {
        SIBLING_SPACING_X
    } else {
        2
    }
}

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

/// Insert `new_node` as a sibling of `selected_id` without altering existing
/// parent-child relationships.
///
/// The new node inherits the parent of `selected_id` and is appended to that
/// parent's child list. The original node hierarchy remains unchanged.
pub fn insert_sibling(nodes: &mut NodeMap, selected_id: NodeID, mut new_node: Node) {
    if !nodes.contains_key(&selected_id) {
        return;
    }
    let parent_id = nodes.get(&selected_id).and_then(|n| n.parent);
    new_node.parent = parent_id;
    let nid = new_node.id;
    nodes.insert(nid, new_node);
    if let Some(pid) = parent_id {
        if let Some(p) = nodes.get_mut(&pid) {
            if !p.children.contains(&nid) {
                p.children.push(nid);
            }
        }
    }
    if let Some(pid) = parent_id {
        println!("SIBLING_OK: parent_id={}", pid);
    } else {
        println!("SIBLING_OK: parent_id=ROOT");
    }
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
            child_x += lane_spacing();
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

/// Overall validity level for a node layout.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum LayoutStatus {
    Valid,
    Overlap,
    OutOfBounds,
}

/// Result of a layout validation pass.
pub struct LayoutCheck {
    pub status: LayoutStatus,
    pub offenders: Vec<NodeID>,
}

/// Validate a mindmap layout. Returns the status and offending node IDs.
pub fn validate_layout(nodes: &NodeMap, area: Rect) -> LayoutCheck {
    let overflow = detect_overflow(nodes, area);
    if !overflow.is_empty() {
        return LayoutCheck { status: LayoutStatus::OutOfBounds, offenders: overflow };
    }
    let collisions = detect_collisions(nodes);
    if !collisions.is_empty() {
        return LayoutCheck { status: LayoutStatus::Overlap, offenders: collisions };
    }
    LayoutCheck { status: LayoutStatus::Valid, offenders: Vec::new() }
}

/// Clamp node coordinates so all entries remain within the provided area.
///
/// When switching between auto and manual layout modes it's possible for nodes
/// to inherit invalid positions (0,0) or fall outside the visible grid. This
/// helper normalizes every node so manual mode always starts from sane bounds.
pub fn clamp_nodes(nodes: &mut NodeMap, area: Rect) {
    let min_x = area.x as i16 + 1;
    let min_y = GEMX_HEADER_HEIGHT.max(area.y as i16 + 1);
    let max_x = area.right() as i16 - 2;
    let max_y = area.bottom() as i16 - 2;

    for node in nodes.values_mut() {
        if node.x == 0 && node.y == 0 {
            node.x = min_x;
            node.y = min_y;
        }
        node.x = node.x.clamp(min_x, max_x);
        node.y = node.y.clamp(min_y, max_y);
    }
}
