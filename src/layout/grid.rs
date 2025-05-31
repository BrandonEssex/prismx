use crate::node::{NodeID, NodeMap};
use crate::layout::{label_bounds, SIBLING_SPACING_X, MIN_SIBLING_SPACING_X};
use crate::theme::layout::spacing_scale;
use ratatui::layout::Rect;
use std::collections::HashMap;

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
