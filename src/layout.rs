use std::collections::HashMap;
use crate::node::{NodeID, NodeMap};
use crate::gemx::layout::{spacing_for, SpacingProfile};

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
    profile: SpacingProfile,
) -> HashMap<NodeID, Coords> {
    let mut map = HashMap::new();
    layout_recursive(nodes, root_id, start_x, start_y, profile, &mut map);
    map
}

/// Internal layout logic for x/y assignment
fn layout_recursive(
    nodes: &NodeMap,
    node_id: NodeID,
    x: u16,
    y: u16,
    profile: SpacingProfile,
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

    let spacing = spacing_for(profile);

    let mut current_y = y + 1;

    for child_id in &node.children {
        current_y = layout_recursive(nodes, *child_id, x + spacing.x as u16, current_y, profile, out) + 1;
    }

    current_y - 1
}
