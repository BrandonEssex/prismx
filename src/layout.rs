use std::collections::HashMap;
use crate::node::{NodeID, NodeMap};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Coords {
    pub x: i16,
    pub y: i16,
}

// Layout spacing constants
const SIBLING_SPACING_X: i16 = 3;
const CHILD_SPACING_Y: i16 = 2;

/// Recursively assigns (x, y) positions to nodes based on void-rs style
pub fn layout_nodes(
    nodes: &NodeMap,
    root_id: NodeID,
    start_x: i16,
    start_y: i16,
) -> HashMap<NodeID, Coords> {
    let mut map = HashMap::new();
    layout_recursive(nodes, root_id, start_x, start_y, &mut map);
    map
}

/// Internal layout logic that balances siblings and stacks children
fn layout_recursive(
    nodes: &NodeMap,
    node_id: NodeID,
    x: i16,
    y: i16,
    out: &mut HashMap<NodeID, Coords>,
) -> i16 {
    out.insert(node_id, Coords { x, y });

    let node = match nodes.get(&node_id) {
        Some(n) => n,
        None => return y,
    };

    if node.collapsed || node.children.is_empty() {
        return y;
    }

    let child_count = node.children.len();
    let mid_index = child_count as i16 / 2;
    let mut max_y = y;

    for (i, child_id) in node.children.iter().enumerate() {
        let offset_x = (i as i16 - mid_index) * SIBLING_SPACING_X;
        let child_x = x + offset_x;
        let child_y = y + CHILD_SPACING_Y;

        let branch_max_y = layout_recursive(nodes, *child_id, child_x, child_y, out);
        max_y = max_y.max(branch_max_y);
    }

    max_y
}
