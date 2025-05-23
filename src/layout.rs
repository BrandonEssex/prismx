use std::collections::HashMap;
use crate::node::{NodeID, NodeMap};

/// Minimum vertical gap between nodes in layout units
pub const MIN_NODE_GAP: u16 = 1;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Coords {
    pub x: u16,
    pub y: u16,
}

/// Recursively assigns (x, y) positions to nodes based on depth
pub fn layout_nodes(
    nodes: &NodeMap,
    root_id: NodeID,
    start_x: u16,
    start_y: u16,
) -> HashMap<NodeID, Coords> {
    let mut map = HashMap::new();
    layout_recursive(nodes, root_id, start_x, start_y, &mut map);
    map
}

/// Internal layout logic for x/y assignment
fn layout_recursive(
    nodes: &NodeMap,
    node_id: NodeID,
    x: u16,
    y: u16,
    out: &mut HashMap<NodeID, Coords>,
) -> u16 {
    out.insert(node_id, Coords { x, y });

    let node = match nodes.get(&node_id) {
        Some(n) => n,
        None => return y,
    };

    if node.collapsed || node.children.is_empty() {
        return y;
    }

    // increment by minimum gap to prevent visual collisions when scaled
    let mut current_y = y + MIN_NODE_GAP;

    for child_id in &node.children {
        // use layout units for spacing; horizontal indent grows by 1 unit
        current_y = layout_recursive(nodes, *child_id, x + 1, current_y, out) + MIN_NODE_GAP;
    }

    current_y - MIN_NODE_GAP
}
