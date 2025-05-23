use std::collections::{HashMap, HashSet};
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

/// Ensure manual positions are unique and use layout defaults when unset
pub fn ensure_manual_positions(nodes: &mut NodeMap, roots: &[NodeID]) {
    // build auto layout to use as fallback
    let mut layout = HashMap::new();
    let mut row = 1;
    for &root_id in roots {
        let l = layout_nodes(nodes, root_id, 2, row);
        let max_y = l.values().map(|c| c.y).max().unwrap_or(row);
        layout.extend(l);
        row = max_y.saturating_add(CHILD_SPACING_Y);
    }

    // assign unique coordinates
    let mut used = HashSet::new();
    for (&id, node) in nodes.iter_mut() {
        let pos = (node.x, node.y);
        if pos == (0, 0) || used.contains(&pos) {
            if let Some(&Coords { x, y }) = layout.get(&id) {
                node.x = x as i16;
                node.y = y as i16;
            }
        }
        used.insert((node.x, node.y));
    }
}
