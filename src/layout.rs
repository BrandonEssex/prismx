use std::collections::HashMap;
use crate::node::{NodeID, NodeMap};
use crate::gemx::layout::{SIBLING_SPACING_X, CHILD_SPACING_Y};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Coords {
    pub x: i16,
    pub y: i16,
}

/// Recursively assigns (x, y) positions to nodes based on depth
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

/// Internal layout logic for x/y assignment
fn layout_recursive(
    nodes: &NodeMap,
    node_id: NodeID,
    x: i16,
    y: i16,
    out: &mut HashMap<NodeID, Coords>,
) {
    out.insert(node_id, Coords { x, y });

    let node = match nodes.get(&node_id) {
        Some(n) => n,
        None => return,
    };

    if node.collapsed || node.children.is_empty() {
        return;
    }

    let sibling_count = node.children.len();
    let mid = sibling_count / 2;

    for (i, &child_id) in node.children.iter().enumerate() {
        let offset_x = (i as i16 - mid as i16) * SIBLING_SPACING_X;
        let child_x = x + offset_x;
        let child_y = y + CHILD_SPACING_Y;
        layout_recursive(nodes, child_id, child_x, child_y, out);
    }
}
