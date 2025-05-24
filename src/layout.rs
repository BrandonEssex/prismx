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

/// Determine the width and height of a node's subtree.
/// Width is measured from the start of the leftmost child subtree to the end of
/// the rightmost child subtree including minimal spacing. Height is the total
/// vertical span of the subtree.
fn get_subtree_span(nodes: &NodeMap, node_id: NodeID) -> (i16, i16) {
    let node = match nodes.get(&node_id) {
        Some(n) => n,
        None => return (0, 0),
    };

    // Base width is approximately the label width plus a small prefix for
    // connectors and selection markers.
    let label_width = node.label.len() as i16 + 2;

    if node.collapsed || node.children.is_empty() {
        return (label_width, 1);
    }

    let mut total_width = 0;
    let mut max_child_height = 0;
    for (i, child) in node.children.iter().enumerate() {
        let (cw, ch) = get_subtree_span(nodes, *child);
        if i > 0 {
            total_width += SIBLING_SPACING_X;
        }
        total_width += cw;
        max_child_height = max_child_height.max(ch);
    }

    let width = label_width.max(total_width);
    let height = 1 + CHILD_SPACING_Y + max_child_height;
    (width, height)
}

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

    // Compute spans for each child subtree so siblings can be spaced without
    // overlap.
    let mut spans = Vec::new();
    let mut total_width = 0;
    for (i, child_id) in node.children.iter().enumerate() {
        let (cw, _) = get_subtree_span(nodes, *child_id);
        if i > 0 {
            total_width += SIBLING_SPACING_X;
        }
        total_width += cw.max(1);
        spans.push((*child_id, cw.max(1))); 
    }

    let mut max_y = y;
    if !spans.is_empty() {
        // Start laying out children so the entire cluster is centered around the
        // parent node's x coordinate.
        let mut cursor_x = x - total_width / 2;
        for (child_id, span_width) in spans {
            let child_x = cursor_x;
            let child_y = y + CHILD_SPACING_Y;

        // println!(
        //     "├── child {} of {} at x={}, y={}",
        //     child_id, node_id, child_x, child_y
        // );

            let branch_max_y = layout_recursive_safe(
                nodes,
                child_id,
                child_x,
                child_y,
                term_width,
                out,
                visited,
                depth + 1,
            );
            max_y = max_y.max(branch_max_y);
            cursor_x += span_width + SIBLING_SPACING_X;
        }
    }

    max_y
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::node::Node;

    fn build_basic_nodes() -> NodeMap {
        let mut map = NodeMap::new();

        let mut root = Node::new(1, "Root", None);
        root.children = vec![2, 3];

        let child_short = Node::new(2, "A", Some(1));
        let child_long = Node::new(3, "VeryLongLabelHere", Some(1));

        map.insert(1, root);
        map.insert(2, child_short);
        map.insert(3, child_long);
        map
    }

    #[test]
    fn siblings_respect_subtree_width() {
        let nodes = build_basic_nodes();
        let layout = layout_nodes(&nodes, 1, 0, 200);

        let left = layout.get(&2).unwrap();
        let right = layout.get(&3).unwrap();

        let (left_span, _) = get_subtree_span(&nodes, 2);
        assert!(right.x >= left.x + left_span + SIBLING_SPACING_X);
    }

    #[test]
    fn nested_subtree_spacing() {
        let mut nodes = build_basic_nodes();

        let mut deep_parent = nodes.get_mut(&2).unwrap();
        deep_parent.children = vec![4];

        let deep_child = Node::new(4, "DeepDeepDeep", Some(2));
        nodes.insert(4, deep_child);

        let layout = layout_nodes(&nodes, 1, 0, 200);

        let left = layout.get(&2).unwrap();
        let right = layout.get(&3).unwrap();

        let (left_span, _) = get_subtree_span(&nodes, 2);
        assert!(right.x >= left.x + left_span + SIBLING_SPACING_X);
    }
}
