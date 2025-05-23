use std::collections::HashMap;
use crate::node::{NodeID, NodeMap};

/// Vertical spacing between parent and child when auto-arranging
pub const BASE_CHILD_SPACING_Y: u16 = 3;

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
    // record current node position
    out.insert(node_id, Coords { x, y });

    let node = match nodes.get(&node_id) {
        Some(n) => n,
        None => return y,
    };

    if node.collapsed || node.children.is_empty() {
        return y;
    }

    // vertical stacking of children directly beneath the parent
    let mut current_y = y + BASE_CHILD_SPACING_Y;

    for (idx, child_id) in node.children.iter().enumerate() {
        let child_x = x + (idx as u16 % 2); // slight stagger to avoid perfect overlap
        current_y =
            layout_recursive(nodes, *child_id, child_x, current_y, out) + BASE_CHILD_SPACING_Y;
    }

    current_y - BASE_CHILD_SPACING_Y
}
