use std::collections::HashMap;
use crate::node::{NodeID, NodeMap};

/// Horizontal spacing between siblings when auto-arranging
pub const SIBLING_SPACING_X: u16 = 3;
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

    // position children on the next row and spread siblings horizontally
    let child_y = y + CHILD_SPACING_Y;
    let total = node.children.len();
    let mid = total / 2;

    let mut max_y = y;
    for (idx, child_id) in node.children.iter().enumerate() {
        let dx = (idx as i32 - mid as i32) * SIBLING_SPACING_X as i32;
        let child_x = if dx < 0 {
            x.saturating_sub((-dx) as u16)
        } else {
            x.saturating_add(dx as u16)
        };
        let bottom = layout_recursive(nodes, *child_id, child_x, child_y, out);
        max_y = max_y.max(bottom);
    }

    max_y
}
