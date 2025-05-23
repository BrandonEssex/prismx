use std::collections::HashMap;
use crate::node::{NodeID, NodeMap};

/// Horizontal spacing between siblings when auto-arranging
pub const SIBLING_SPACING_X: i16 = 3;
/// Vertical spacing between parent and child when auto-arranging
pub const CHILD_SPACING_Y: u16 = 2;

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

    let mid = node.children.len() / 2;
    let mut max_y = y;

    for (idx, child_id) in node.children.iter().enumerate() {
        let offset = (idx as i16 - mid as i16) * SIBLING_SPACING_X;
        let child_x_i = x as i16 + offset;
        let child_x = child_x_i.max(0) as u16;
        let child_y = y + CHILD_SPACING_Y;
        let bottom = layout_recursive(nodes, *child_id, child_x, child_y, out);
        if bottom > max_y {
            max_y = bottom;
        }
    }

    max_y
}
