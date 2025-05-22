use std::collections::HashMap;
use crate::node::{Node, NodeID};

#[derive(Copy, Clone, Debug)]
pub struct Coords {
    pub x: u16,
    pub y: u16,
}

/// Horizontally auto-arranges nodes into a visual grid layout.
pub fn layout_nodes(
    nodes: &HashMap<NodeID, Node>,
    root: NodeID,
    offset_x: u16,
    offset_y: u16,
) -> HashMap<NodeID, Coords> {
    let mut positions = HashMap::new();
    let mut max_y = offset_y;

    fn layout_recursive(
        id: NodeID,
        nodes: &HashMap<NodeID, Node>,
        x: &mut u16,
        y: &mut u16,
        positions: &mut HashMap<NodeID, Coords>,
        depth: u16,
    ) {
        let mut current_y = *y;
        *y += 2;

        let node = &nodes[&id];

        if node.collapsed || node.children.is_empty() {
            positions.insert(id, Coords { x: *x, y: current_y });
            return;
        }

        let start_x = *x;
        for &child_id in &node.children {
            layout_recursive(child_id, nodes, x, y, positions, depth + 1);
            *x += 2;
        }

        let mid_x = (*x + start_x).saturating_sub(2) / 2;
        positions.insert(id, Coords { x: mid_x, y: current_y });
    }

    let mut x = offset_x;
    for &root_id in nodes[&root].children.iter() {
        layout_recursive(root_id, nodes, &mut x, &mut max_y, &mut positions, 0);
        x += 4; // spacing between subtrees
    }

    positions.insert(root, Coords { x: offset_x, y: offset_y }); // place root at origin
    positions
}
