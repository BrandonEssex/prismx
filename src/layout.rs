use std::collections::{HashMap, HashSet};
use crate::node::{NodeID, NodeMap};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Coords {
    pub x: i16,
    pub y: i16,
}

pub const SIBLING_SPACING_X: i16 = 3;
pub const CHILD_SPACING_Y: i16 = 1;
pub const FREE_GRID_COLUMNS: usize = 4;
pub const GEMX_HEADER_HEIGHT: i16 = 2;
pub const MAX_LAYOUT_DEPTH: usize = 50;

/// Public layout function
pub fn layout_nodes(
    nodes: &NodeMap,
    root_id: NodeID,
    start_y: i16,
    term_width: i16,
) -> HashMap<NodeID, Coords> {
    let start_y = start_y.max(GEMX_HEADER_HEIGHT + 1);
    let start_x = term_width / 2;
    let mut coords = HashMap::new();
    let mut visited = HashSet::new();
    layout_recursive_safe(
        nodes,
        root_id,
        start_x,
        start_y,
        term_width,
        &mut coords,
        &mut visited,
        0,
    );
    coords
}

fn layout_recursive_safe(
    nodes: &NodeMap,
    node_id: NodeID,
    x: i16,
    y: i16,
    term_width: i16,
    out: &mut HashMap<NodeID, Coords>,
    visited: &mut HashSet<NodeID>,
    depth: usize,
) -> i16 {
    if !visited.insert(node_id) {
        println!("⚠️  Cycle detected: {}", node_id);
        return y;
    }

    if depth > MAX_LAYOUT_DEPTH {
        println!("🛑 Max layout depth exceeded at node {}", node_id);
        return y;
    }

    out.insert(node_id, Coords { x, y });
    let node = match nodes.get(&node_id) {
        Some(n) => n,
        None => return y,
    };

    // println!(
    //     "📐 layout: node {} at x={}, y={}, children={:?}",
    //     node_id, x, y, node.children
    // );

    if node.collapsed || node.children.is_empty() {
        return y;
    }

    let child_count = node.children.len();
    let mid = child_count / 2;
    let mut spacing_x = SIBLING_SPACING_X;
    if child_count > 1 {
        let needed = (child_count as i16 - 1) * spacing_x;
        if needed > term_width {
            spacing_x = (term_width / (child_count as i16)).max(1);
        }
    }
    let mut max_y = y;

    for (i, child_id) in node.children.iter().enumerate() {
        let offset_x = (i as i16 - mid as i16) * spacing_x;
        let child_x = x + offset_x;
        let child_y = y + CHILD_SPACING_Y;

        // println!(
        //     "├── child {} of {} at x={}, y={}",
        //     child_id, node_id, child_x, child_y
        // );

        let branch_max_y = layout_recursive_safe(
            nodes,
            *child_id,
            child_x,
            child_y,
            term_width,
            out,
            visited,
            depth + 1,
        );

        max_y = max_y.max(branch_max_y);
    }

    max_y
}
