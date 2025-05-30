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

/// Depth at which additional horizontal spacing is applied for long branches.
pub const DEEP_BRANCH_THRESHOLD: usize = 12;
/// Horizontal offset step applied for each level beyond the threshold.
pub const DEEP_BRANCH_STEP_X: i16 = 2;
use std::collections::HashMap;

/// Recursively position nodes so siblings are laid out horizontally
/// while children stack vertically beneath their parent.
///
/// Layout respects each subtree's span to prevent overlap.
pub fn layout_vertical(nodes: &mut NodeMap, root: NodeID, spacing_y: i16) {
    fn depth_offset(depth: usize) -> i16 {
        if depth > DEEP_BRANCH_THRESHOLD {
            ((depth - DEEP_BRANCH_THRESHOLD) as i16) * DEEP_BRANCH_STEP_X
        } else {
            0
        }
    }

    fn layout_subtree(
        nodes: &mut NodeMap,
        id: NodeID,
        x: i16,
        y: i16,
        spacing_y: i16,
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
            let _ = node;

            if collapsed || children.is_empty() {
                return span.max(label_w);
            }

            let mut child_x = x;
            let len = children.len();
            for (i, child) in children.iter().copied().enumerate() {
                let cspan = subtree_span(nodes, child);
                layout_subtree(nodes, child, child_x, y + spacing_y, spacing_y, depth + 1);
                let label_w_child = nodes
                    .get(&child)
                    .map(|c| label_bounds(&c.label).0)
                    .unwrap_or(2);
                let mut child_w = cspan.max(label_w_child + MIN_NODE_GAP);
                child_w += depth_offset(depth + 1);
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
    layout_subtree(nodes, root, start_x, start_y, spacing_y, 0);
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
    // Default horizontal spacing between siblings
    let mut off = SIBLING_SPACING_X;

    // When clusters grow large, compress the spacing to avoid overflow.
    if len > 8 {
        let ratio = 8.0 / len as f32;
        off = ((SIBLING_SPACING_X as f32) * ratio).ceil() as i16;
        if off < MIN_SIBLING_SPACING_X {
            off = MIN_SIBLING_SPACING_X;
        }
    }

    // Small buffer every four siblings to reduce visual crowding.
    if len > 4 && (index + 1) % 4 == 0 {
        off += MIN_SIBLING_SPACING_X;
    }

    off
}

/// Reflow an existing set of siblings so they remain horizontally aligned
/// relative to their parent. This is a lightweight helper used by the mindmap
/// module when dynamically inserting nodes. Only the sibling positions are
/// mutated; the parent node remains fixed in place.
pub fn reflow_siblings(nodes: &mut NodeMap, parent: NodeID, spacing_y: i16) {
    let Some(p) = nodes.get(&parent).cloned() else { return };
    if p.children.is_empty() || p.collapsed {
        return;
    }

    let spacing_y = spacing_y.max(MIN_CHILD_SPACING_Y);

    // Calculate the left anchor based on the parent's center and total span
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
            child_x += sibling_offset(i, len);
        }
    }
}

use ratatui::layout::Rect;
/// Return the bounding grid size (rows, columns) required to fit all nodes.
pub fn grid_size(nodes: &NodeMap) -> (i16, i16) {
    if nodes.is_empty() {
        return (0, 0);
    }
    let mut min_x = i16::MAX;
    let mut min_y = i16::MAX;
    let mut max_x = i16::MIN;
    let mut max_y = i16::MIN;
    for n in nodes.values() {
        let (w, h) = label_bounds(&n.label);
        min_x = min_x.min(n.x);
        min_y = min_y.min(n.y);
        max_x = max_x.max(n.x + w - 1);
        max_y = max_y.max(n.y + h - 1);
    }
    (max_y - min_y + 1, max_x - min_x + 1)
}

/// Detect nodes that fall outside the provided area.
pub fn detect_overflow(nodes: &NodeMap, area: Rect) -> Vec<NodeID> {
    let mut out = Vec::new();
    for (&id, node) in nodes {
        let (w, h) = label_bounds(&node.label);
        let left = node.x;
        let right = node.x + w - 1;
        let top = node.y;
        let bottom = node.y + h - 1;
        if left < area.x as i16
            || top < area.y as i16
            || right > area.right() as i16 - 1
            || bottom > area.bottom() as i16 - 1
        {
            out.push(id);
        }
    }
    out
}

/// Detect any overlapping node labels and return the offending IDs.
pub fn detect_collisions(nodes: &NodeMap) -> Vec<NodeID> {
    use std::collections::HashMap;
    let mut grid: HashMap<(i16, i16), NodeID> = HashMap::new();
    let mut bad = Vec::new();
    for (&id, node) in nodes {
        let (w, h) = label_bounds(&node.label);
        for dx in 0..w {
            for dy in 0..h {
                let key = (node.x + dx, node.y + dy);
                if let Some(prev) = grid.get(&key) {
                    if !bad.contains(&id) {
                        bad.push(id);
                    }
                    if !bad.contains(prev) {
                        bad.push(*prev);
                    }
                } else {
                    grid.insert(key, id);
                }
            }
        }
    }
    bad
}
