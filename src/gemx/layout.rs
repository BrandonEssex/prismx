use crate::node::{NodeID, NodeMap};
use crate::state::AppState;
use std::collections::hash_map::DefaultHasher;
use std::collections::HashSet;
use std::hash::{Hash, Hasher};
use std::sync::atomic::{AtomicU8, Ordering};

/// Layout mode selector for GemX.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum LayoutMode {
    Tree = 0,
    Grid = 1,
    Hybrid = 2,
}

static MODE: AtomicU8 = AtomicU8::new(LayoutMode::Tree as u8);

/// Get the currently active layout mode.
pub fn current_mode() -> LayoutMode {
    match MODE.load(Ordering::Relaxed) {
        1 => LayoutMode::Grid,
        2 => LayoutMode::Hybrid,
        _ => LayoutMode::Tree,
    }
}

/// Cycle to the next layout mode.
pub fn toggle_mode() {
    let next = (MODE.load(Ordering::Relaxed) + 1) % 3;
    MODE.store(next, Ordering::Relaxed);
}

/// Set the layout mode explicitly.
pub fn set_mode(mode: LayoutMode) {
    MODE.store(mode as u8, Ordering::Relaxed);
}


fn graph_fingerprint(nodes: &NodeMap) -> (usize, u64) {
    let mut pairs: Vec<(NodeID, Option<NodeID>)> =
        nodes.iter().map(|(&id, n)| (id, n.parent)).collect();
    pairs.sort_by_key(|p| p.0);
    let mut hasher = DefaultHasher::new();
    pairs.hash(&mut hasher);
    (pairs.len(), hasher.finish())
}

/// Fallback centering with recursion depth guard.
pub fn fallback_center(state: &mut AppState, root: NodeID, depth: usize) {
    if depth > crate::layout::MAX_DEPTH {
        crate::log_warn!("fallback_center recursion halted at {}", depth);
        return;
    }
    if let Some(node) = state.nodes.get(&root) {
        state.scroll_x = node.x;
        state.scroll_y = node.y;
    }
}

/// Arrange a simple horizontal row of nodes.
pub fn arrange_horizontally(nodes: &mut NodeMap, ids: &[NodeID], y: i16) {
    let mut x = 0;
    for id in ids {
        if let Some(n) = nodes.get_mut(id) {
            n.x = x;
            n.y = y;
            x += crate::layout::SIBLING_SPACING_X;
        }
    }
}

/// Simple overlap clamp used by the experimental layout modes.
fn clamp_overlaps(nodes: &mut NodeMap) {
    let mut used = HashSet::new();
    for node in nodes.values_mut() {
        let mut pos = (node.x, node.y);
        while used.contains(&pos) {
            pos.0 += crate::layout::SIBLING_SPACING_X;
        }
        node.x = pos.0;
        node.y = pos.1;
        used.insert(pos);
    }
}

/// Apply layout to the provided nodes according to the current mode.
pub fn apply_layout(state: &mut AppState) {
    let nodes = &mut state.nodes;
    let roots = state.root_nodes.clone();
    let key = graph_fingerprint(nodes);
    if state.layout_key == key {
        return;
    }
    state.layout_key = key;
    match current_mode() {
        LayoutMode::Tree => {
            let mut y = crate::layout::GEMX_HEADER_HEIGHT + 1;
            for &id in &roots {
                arrange_horizontally(nodes, &[id], y);
                y += crate::layout::CHILD_SPACING_Y;
            }
        }
        LayoutMode::Grid => {
            let cols = crate::layout::FREE_GRID_COLUMNS as i16;
            let mut i = 0i16;
            for &id in &roots {
                if let Some(n) = nodes.get_mut(&id) {
                    n.x = (i % cols) * crate::layout::SIBLING_SPACING_X;
                    n.y = (i / cols) * crate::layout::CHILD_SPACING_Y
                        + crate::layout::GEMX_HEADER_HEIGHT
                        + 1;
                    i += 1;
                }
            }
        }
        LayoutMode::Hybrid => {
            let cols = crate::layout::FREE_GRID_COLUMNS as i16;
            let mut i = 0i16;
            for &id in &roots {
                if let Some(n) = nodes.get_mut(&id) {
                    n.x = (i % cols) * crate::layout::SIBLING_SPACING_X;
                    n.y = (i / cols) * crate::layout::CHILD_SPACING_Y
                        + crate::layout::GEMX_HEADER_HEIGHT
                        + 1;
                    let child_ids = n.children.clone();
                    let child_y = n.y;
                    i += 1;
                    let _ = n;
                    arrange_horizontally(
                        nodes,
                        &child_ids,
                        child_y + crate::layout::CHILD_SPACING_Y,
                    );
                }
            }
        }
    }

    clamp_overlaps(nodes);
}
