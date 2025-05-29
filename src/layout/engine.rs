use crate::node::{NodeID, NodeMap};
use crate::layout::{label_bounds, subtree_span, CHILD_SPACING_Y, SIBLING_SPACING_X};
use crate::theme::layout::spacing_scale;
use std::collections::HashMap;

/// Recursively position nodes so siblings are laid out horizontally
/// while children stack vertically beneath their parent.
///
/// Layout respects each subtree's span to prevent overlap.
pub fn layout_vertical(nodes: &mut NodeMap, root: NodeID) {
    fn layout_subtree(nodes: &mut NodeMap, id: NodeID, x: i16, y: i16) -> i16 {
        let span = subtree_span(nodes, id);
        let (label_w, _) = nodes
            .get(&id)
            .map(|n| label_bounds(&n.label))
            .unwrap_or((2, 1));

        if let Some(node) = nodes.get_mut(&id) {
            node.x = x + (span - label_w) / 2;
            node.y = y;
            let children = node.children.clone();
            let collapsed = node.collapsed;
            let _ = node;

            if collapsed || children.is_empty() {
                return span.max(label_w);
            }

            let mut child_x = x;
            let len = children.len();
            for (i, child) in children.iter().copied().enumerate() {
                let cspan = subtree_span(nodes, child);
                layout_subtree(nodes, child, child_x, y + CHILD_SPACING_Y);
                child_x += cspan;
                if i + 1 < len {
                    child_x += SIBLING_SPACING_X;
                }
            }
        }
        span
    }

    let start_x = nodes.get(&root).map(|n| n.x).unwrap_or(0);
    let start_y = nodes.get(&root).map(|n| n.y).unwrap_or(0);
    layout_subtree(nodes, root, start_x, start_y);
}

/// Calculate the depth of each subtree.
pub fn compute_depths(nodes: &NodeMap) -> HashMap<NodeID, usize> {
    fn depth(nodes: &NodeMap, id: NodeID) -> usize {
        if let Some(n) = nodes.get(&id) {
            if n.children.is_empty() {
                1
            } else {
                1 + n.children.iter().map(|c| depth(nodes, *c)).max().unwrap_or(0)
            }
        } else {
            0
        }
    }

    let mut map = HashMap::new();
    for id in nodes.keys().copied() {
        map.insert(id, depth(nodes, id));
    }
    map
}

/// Public wrapper used by renderers to compute character spacing
/// for a given zoom level.
pub fn spacing_for_zoom(zoom: f32) -> (i16, i16) {
    spacing_scale(zoom)
}
