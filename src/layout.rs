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
    let _ = layout_recursive_safe(
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
) -> (i16, i16, i16) {
    if !visited.insert(node_id) {
        println!("⚠️  Cycle detected: {}", node_id);
        return (y, x, x);
    }

    if depth > MAX_LAYOUT_DEPTH {
        println!("🛑 Max layout depth exceeded at node {}", node_id);
        return (y, x, x);
    }

    let node = match nodes.get(&node_id) {
        Some(n) => n,
        None => return (y, x, x),
    };

    // println!(
    //     "📐 layout: node {} at x={}, y={}, children={:?}",
    //     node_id, x, y, node.children
    // );

    if node.collapsed || node.children.is_empty() {
        out.insert(node_id, Coords { x, y });
        return (y, x, x + node.label.len() as i16 + 2);
    }

    // First pass: layout children temporarily to determine their spans
    let mut spans = Vec::new();
    let mut total_width = 0;
    let mut max_y = y;
    let mut min_x_span = x;
    let mut max_x_span = x + node.label.len() as i16 + 2;

    for (i, child_id) in node.children.iter().enumerate() {
        let child_y = y + CHILD_SPACING_Y;
        let (branch_max_y, branch_min_x, branch_max_x) = layout_recursive_safe(
            nodes,
            *child_id,
            x,
            child_y,
            term_width,
            out,
            visited,
            depth + 1,
        );
        let span_width = branch_max_x - branch_min_x;
        spans.push((*child_id, span_width, branch_min_x, branch_max_x));
        if i > 0 {
            total_width += SIBLING_SPACING_X;
        }
        total_width += span_width.max(1);
        max_y = max_y.max(branch_max_y);
    }

    if !spans.is_empty() {
        // Second pass: position each child relative to the overall width
        let mut cursor_x = x - total_width / 2;
        for (child_id, span_width, branch_min_x, branch_max_x) in spans {
            let current_x = out.get(&child_id).map(|c| c.x).unwrap_or(x);
            let dx = cursor_x - current_x;
            shift_subtree(child_id, dx, out, nodes);
            min_x_span = min_x_span.min(branch_min_x + dx);
            max_x_span = max_x_span.max(branch_max_x + dx);
            cursor_x += span_width.max(1) + SIBLING_SPACING_X;
        }

        // Recenter the parent horizontally above its children.
        #[cfg(feature = "manual_coords")]
        let has_manual = node.manual_coords.is_some();
        #[cfg(not(feature = "manual_coords"))]
        let has_manual = false;

        if has_manual {
            out.insert(node_id, Coords { x, y });
        } else {
            let new_x = (min_x_span + max_x_span) / 2;
            out.insert(node_id, Coords { x: new_x, y });
            min_x_span = min_x_span.min(new_x);
            max_x_span = max_x_span.max(new_x + node.label.len() as i16 + 2);
        }
    }

    (max_y, min_x_span, max_x_span)
}

fn shift_subtree(id: NodeID, dx: i16, out: &mut HashMap<NodeID, Coords>, nodes: &NodeMap) {
    if dx == 0 {
        return;
    }
    if let Some(pos) = out.get_mut(&id) {
        pos.x += dx;
    }
    if let Some(node) = nodes.get(&id) {
        if !node.collapsed {
            for child in &node.children {
                shift_subtree(*child, dx, out, nodes);
            }
        }
    }
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

    fn subtree_bounds(nodes: &NodeMap, layout: &HashMap<NodeID, Coords>, id: NodeID) -> (i16, i16) {
        fn walk(nodes: &NodeMap, layout: &HashMap<NodeID, Coords>, id: NodeID, mi: &mut i16, ma: &mut i16) {
            if let Some(n) = nodes.get(&id) {
                if let Some(c) = layout.get(&id) {
                    let w = n.label.len() as i16 + 2;
                    *mi = (*mi).min(c.x);
                    *ma = (*ma).max(c.x + w);
                    if !n.collapsed {
                        for child in &n.children {
                            walk(nodes, layout, *child, mi, ma);
                        }
                    }
                }
            }
        }

        let mut mi = i16::MAX;
        let mut ma = i16::MIN;
        walk(nodes, layout, id, &mut mi, &mut ma);
        (mi, ma)
    }

    #[test]
    fn subtree_packing_prevents_overlap() {
        let mut nodes = NodeMap::new();

        let mut root = Node::new(1, "Root", None);
        root.children = vec![2, 3];

        let mut left = Node::new(2, "L", Some(1));
        left.children = vec![4, 5];

        let right = Node::new(3, "R", Some(1));
        let g1 = Node::new(4, "WideWideWide", Some(2));
        let g2 = Node::new(5, "WideWideWide", Some(2));

        nodes.insert(1, root);
        nodes.insert(2, left);
        nodes.insert(3, right);
        nodes.insert(4, g1);
        nodes.insert(5, g2);

        let layout = layout_nodes(&nodes, 1, 0, 200);

        let (_, left_max) = subtree_bounds(&nodes, &layout, 2);
        let right_x = layout[&3].x;

        assert!(right_x >= left_max + SIBLING_SPACING_X);
    }

    #[test]
    fn parent_recenters_above_children() {
        let mut nodes = NodeMap::new();

        let mut root = Node::new(1, "Root", None);
        root.children = vec![2, 3];

        let mut left = Node::new(2, "L", Some(1));
        left.children = vec![4];

        let right = Node::new(3, "R", Some(1));
        let grand = Node::new(4, "ExtremelyWideLabelHere", Some(2));

        nodes.insert(1, root);
        nodes.insert(2, left);
        nodes.insert(3, right);
        nodes.insert(4, grand);

        let layout = layout_nodes(&nodes, 1, 0, 200);

        let root_x = layout[&1].x;

        fn bounds(nodes: &NodeMap, layout: &HashMap<NodeID, Coords>, id: NodeID, mi: &mut i16, ma: &mut i16) {
            if let Some(n) = nodes.get(&id) {
                if let Some(c) = layout.get(&id) {
                    let w = n.label.len() as i16 + 2;
                    *mi = (*mi).min(c.x);
                    *ma = (*ma).max(c.x + w);
                    if !n.collapsed {
                        for child in &n.children {
                            bounds(nodes, layout, *child, mi, ma);
                        }
                    }
                }
            }
        }

        let mut min_x = i16::MAX;
        let mut max_x = i16::MIN;
        for child in &nodes.get(&1).unwrap().children {
            bounds(&nodes, &layout, *child, &mut min_x, &mut max_x);
        }

        let expected = (min_x + max_x) / 2;

        assert_eq!(root_x, expected);
    }
}
