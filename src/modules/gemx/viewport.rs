use crate::state::AppState;
use crate::node::NodeID;
use crate::layout::{center_on_node, spacing_for_zoom};
use crossterm::terminal;

/// Determine if the given node is currently visible on screen.
pub fn node_visible(state: &AppState, node_id: NodeID) -> bool {
    let (tw, th) = terminal::size().unwrap_or((80, 20));
    let zoom = state.zoom_scale as f32;
    let (sx, sy) = spacing_for_zoom(state.zoom_scale);
    if let Some(node) = state.nodes.get(&node_id) {
        let dx = ((node.x - state.scroll_x) as f32 * sx as f32 * zoom).round();
        let dy = ((node.y - state.scroll_y) as f32 * sy as f32 * zoom).round();
        return dx >= 0.0 && dx < tw as f32 && dy >= 0.0 && dy < th as f32;
    }
    false
}

/// Scroll the viewport so that `node_id` is visible if needed.
pub fn ensure_visible(state: &mut AppState, node_id: NodeID) {
    if !node_visible(state, node_id) {
        center_on_node(state, node_id);
        state.scroll_target_x = state.scroll_x;
        state.scroll_target_y = state.scroll_y;
    }
}

/// Keep the currently focused node in view.
pub fn follow_focus(state: &mut AppState) {
    if let Some(id) = state.selected {
        ensure_visible(state, id);
    }
}
