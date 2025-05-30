use crate::state::AppState;
use crate::node::NodeID;
use crate::layout::{center_on_node, spacing_for_zoom};
use crossterm::terminal;

/// Determine if the given node is currently visible on screen.
pub fn node_visible(state: &AppState, node_id: NodeID) -> bool {
    let (tw, th) = terminal::size().unwrap_or((80, 20));
    let zoom = state.zoom_scale as f32;
    let (sx, sy) = spacing_for_zoom(state.zoom_scale);

    // When zoomed out it's easy for nodes to hug the terminal edges. Apply a
    // small visibility margin so centering leaves room on the right and bottom
    // of the screen. The top margin keeps the focused node from touching the
    // header.
    let right_pad = if zoom <= 0.3 {
        6.0
    } else if zoom <= 0.5 {
        4.0
    } else {
        2.0
    };
    let bottom_pad = if zoom <= 0.3 {
        3.0
    } else if zoom <= 0.5 {
        2.0
    } else {
        1.0
    };
    let top_pad = if zoom <= 0.5 { 1.0 } else { 0.0 };

    if let Some(node) = state.nodes.get(&node_id) {
        let dx = ((node.x - state.scroll_x) as f32 * sx as f32 * zoom).round();
        let dy = ((node.y - state.scroll_y) as f32 * sy as f32 * zoom).round();
        return dx >= 0.0
            && dx < tw as f32 - right_pad
            && dy >= top_pad
            && dy < th as f32 - bottom_pad;
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
