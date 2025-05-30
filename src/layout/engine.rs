use crate::node::{NodeID, NodeMap};
use crate::layout::{
    label_bounds,
    subtree_span,
    MIN_CHILD_SPACING_Y,
    SIBLING_SPACING_X,
    MIN_SIBLING_SPACING_X,
    MIN_NODE_GAP,
};
use crate::theme::layout::spacing_scale;
use std::collections::HashMap;

/// Recursively position nodes so siblings are laid out horizontally
/// while children stack vertically beneath their parent.
///
/// Layout respects each subtree's span to prevent overlap.
pub fn layout_vertical(nodes: &mut NodeMap, root: NodeID, spacing_y: i16) {
    fn layout_subtree(nodes: &mut NodeMap, id: NodeID, x: i16, y: i16, spacing_y: i16) -> i16 {
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
                layout_subtree(nodes, child, child_x, y + spacing_y, spacing_y);
                let label_w_child = nodes
                    .get(&child)
                    .map(|c| label_bounds(&c.label).0)
                    .unwrap_or(2);
                let child_w = cspan.max(label_w_child + MIN_NODE_GAP);
                child_x += child_w;
                if i + 1 < len {
                    child_x += sibling_offset(i, len);
                }
            }
        }
        span
    }

    let spacing_y = spacing_y.max(MIN_CHILD_SPACING_Y);

    let (label_w, _) = nodes
        .get(&root)
        .map(|n| label_bounds(&n.label))
        .unwrap_or((2, 1));
    let span = subtree_span(nodes, root);

    // Preserve the root's center position so the active tree doesn't jitter
    // when new children are inserted. Calculate the left anchor from the
    // existing center position and span width.
    let root_center = nodes.get(&root).map(|n| n.x).unwrap_or(0);
    let start_x = root_center - (span - label_w) / 2;
    let start_y = nodes.get(&root).map(|n| n.y).unwrap_or(0);
    layout_subtree(nodes, root, start_x, start_y, spacing_y);
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

/// Calculate the horizontal center position of a node's label.
pub fn center_x(nodes: &NodeMap, id: NodeID) -> i16 {
    if let Some(n) = nodes.get(&id) {
        let (w, _) = label_bounds(&n.label);
        n.x + w / 2
    } else {
        0
    }
}

/// Calculate horizontal spacing after sibling `index` within `len` siblings.
///
/// Adds a small buffer every four siblings to reduce visual crowding.
pub fn sibling_offset(index: usize, len: usize) -> i16 {
    let mut off = SIBLING_SPACING_X;
    if len > 4 && (index + 1) % 4 == 0 {
        off += MIN_SIBLING_SPACING_X;
    }
    off
}
