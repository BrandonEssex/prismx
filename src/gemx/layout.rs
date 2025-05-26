use crate::node::{NodeID, NodeMap};
use crate::state::AppState;

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
