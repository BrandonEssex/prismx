use crate::node::{NodeID, NodeMap};
use crate::layout::{
    label_bounds,
    subtree_span,
    MIN_CHILD_SPACING_Y,
    MIN_NODE_GAP,
};
use std::collections::{HashMap, HashSet};

/// Depth at which additional horizontal spacing is applied for long branches.
pub const DEEP_BRANCH_THRESHOLD: usize = 6;
/// Horizontal offset step applied for each level beyond the threshold.
pub const DEEP_BRANCH_STEP_X: i16 = 1;

fn depth_offset(depth: usize) -> i16 {
    if depth > DEEP_BRANCH_THRESHOLD {
        ((depth - DEEP_BRANCH_THRESHOLD) as i16) * DEEP_BRANCH_STEP_X
    } else {
        0
    }
}

/// Recursively position nodes so siblings are laid out horizontally
/// while children stack vertically beneath their parent.
/// Layout respects each subtree's span to prevent overlap.
pub fn layout_vertical(
    nodes: &mut NodeMap,
    root: NodeID,
    spacing_y: i16,
    focus: Option<&HashSet<NodeID>>,
) {
    fn layout_subtree(
        nodes: &mut NodeMap,
        id: NodeID,
        x: i16,
        y: i16,
        base_spacing: i16,
        focus: Option<&HashSet<NodeID>>,
        depth: usize,
    ) -> i16 {
        let span = subtree_span(nodes, id);
        let (label_w, _) = nodes
            .get(&id)
            .map(|n| label_bounds(&n.label))
            .unwrap_or((2, 1));

        if let Some(node) = nodes.get_mut(&id) {
            node.x = x + depth_offset(depth) + (span - label_w) / 2;
            node.y = y;
            let children = node.children.clone();
            let collapsed = node.collapsed;

            if collapsed || children.is_empty() {
                return span.max(label_w);
            }

            let mut child_x = x;
            let len = children.len();
            for (i, child) in children.iter().copied().enumerate() {
                let cspan = subtree_span(nodes, child);
                let mut next_spacing = base_spacing;
                if let Some(set) = focus {
                    if set.contains(&id) {
                        next_spacing = next_spacing.saturating_sub(1);
                    } else if !set.is_empty() {
                        next_spacing += 1;
                    }
                }
                layout_subtree(nodes, child, child_x, y + next_spacing, base_spacing, focus, depth + 1);
                let label_w_child = nodes
                    .get(&child)
                    .map(|c| label_bounds(&c.label).0)
                    .unwrap_or(2);
                let mut child_w = cspan.max(label_w_child + MIN_NODE_GAP);
                child_w += depth_offset(depth + 1);
                child_x += child_w;
                if i + 1 < len {
                    child_x += crate::layout::grid::sibling_offset(i, len);
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

    let root_center = nodes.get(&root).map(|n| n.x).unwrap_or(0);
    let start_x = root_center - (span - label_w) / 2;
    let start_y = nodes.get(&root).map(|n| n.y).unwrap_or(0);
    layout_subtree(nodes, root, start_x, start_y, spacing_y, focus, 0);
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

/// Reflow an existing set of siblings so they remain horizontally aligned
/// relative to their parent. Only the sibling positions are mutated.
pub fn reflow_siblings(nodes: &mut NodeMap, parent: NodeID, spacing_y: i16) {
    let Some(p) = nodes.get(&parent).cloned() else { return };
    if p.children.is_empty() || p.collapsed {
        return;
    }

    let spacing_y = spacing_y.max(MIN_CHILD_SPACING_Y);

    let total_span = subtree_span(nodes, parent);
    let (label_w, _) = label_bounds(&p.label);
    let mut child_x = p.x - (total_span - label_w) / 2;
    let child_y = p.y + spacing_y;
    let len = p.children.len();

    for (i, cid) in p.children.iter().copied().enumerate() {
        let span = subtree_span(nodes, cid);
        let label_w_child = nodes
            .get(&cid)
            .map(|c| label_bounds(&c.label).0)
            .unwrap_or(2);
        let child_w = span.max(label_w_child + MIN_NODE_GAP);

        if let Some(cnode) = nodes.get_mut(&cid) {
            cnode.x = child_x + (span - label_w_child) / 2;
            cnode.y = child_y;
        }

        child_x += child_w;
        if i + 1 < len {
            child_x += crate::layout::grid::sibling_offset(i, len);
        }
    }
}
