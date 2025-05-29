use crate::node::{NodeID, NodeMap};
use crate::layout::{CHILD_SPACING_Y, SIBLING_SPACING_X};

/// Recursively position nodes so children appear beneath their parent.
///
/// Each sibling is nudged horizontally by [`SIBLING_SPACING_X`].
pub fn layout_vertical(nodes: &mut NodeMap, root: NodeID) {
    fn walk(nodes: &mut NodeMap, id: NodeID, x: i16, y: i16) {
        if let Some(node) = nodes.get_mut(&id) {
            node.x = x;
            node.y = y;
            for (i, child) in node.children.clone().into_iter().enumerate() {
                let cx = x + (i as i16) * SIBLING_SPACING_X;
                let cy = y + CHILD_SPACING_Y;
                walk(nodes, child, cx, cy);
            }
        }
    }
    walk(nodes, root, nodes.get(&root).map(|n| n.x).unwrap_or(0), nodes.get(&root).map(|n| n.y).unwrap_or(0));
}

/// Calculate the depth of each subtree.
pub fn compute_depths(nodes: &NodeMap) -> std::collections::HashMap<NodeID, usize> {
    fn depth(nodes: &NodeMap, id: NodeID) -> usize {
        if let Some(n) = nodes.get(&id) {
            if n.children.is_empty() { 1 } else { 1 + n.children.iter().map(|c| depth(nodes, *c)).max().unwrap_or(0) }
        } else { 0 }
    }
    let mut map = std::collections::HashMap::new();
    for id in nodes.keys().copied() {
        map.insert(id, depth(nodes, id));
    }
    map
}
