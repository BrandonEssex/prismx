use std::collections::HashMap;
use crate::node::{NodeID, NodeMap};

pub const SIBLING_SPACING_X: i16 = 3;
pub const CHILD_SPACING_Y: i16 = 2;
pub const FREE_GRID_COLUMNS: i16 = 4;

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
    layout_recursive(nodes, root_id, start_x as i16, start_y as i16, &mut map);
    map
}

/// Internal layout logic for x/y assignment
fn layout_recursive(
    nodes: &NodeMap,
    node_id: NodeID,
    x: i16,
    y: i16,
    out: &mut HashMap<NodeID, Coords>,

) -> i16 {
    out.insert(node_id, Coords { x: x.max(0) as u16, y: y.max(0) as u16 });

    let node = match nodes.get(&node_id) {
        Some(n) => n,
        None => return y,
    };

    if node.collapsed || node.children.is_empty() {
        return y;
    }

    let count = node.children.len() as i16;
    let mid = count / 2;

    let mut max_y = y;

    for (i, child_id) in node.children.iter().enumerate() {
        let offset_x = (i as i16 - mid) * SIBLING_SPACING_X;
        let child_x = x + offset_x;
        let child_y = y + CHILD_SPACING_Y;
        let end_y = layout_recursive(nodes, *child_id, child_x, child_y, out);
        if end_y > max_y {
            max_y = end_y;
        }
    }

    max_y
}
