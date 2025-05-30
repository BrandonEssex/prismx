use crate::state::AppState;
use crate::node::NodeID;
use super::viewport;

/// Ensure the newly inserted node remains visible by centering on it.
pub fn focus_new_node(state: &mut AppState, node_id: NodeID) {
    viewport::ensure_visible(state, node_id);
}

/// Follow the currently selected node each frame.
pub fn follow_focus_node(state: &mut AppState) {
    viewport::follow_focus(state);
}

/// Clamp scroll offsets relative to zoom level to avoid jumpiness.
pub fn clamp_zoom_scroll(state: &mut AppState) {
    let limit = (1000.0 * state.zoom_scale) as i16;
    state.scroll_x = state.scroll_x.clamp(0, limit);
    state.scroll_y = state.scroll_y.clamp(0, limit);
    state.scroll_target_x = state.scroll_target_x.clamp(0, limit);
    state.scroll_target_y = state.scroll_target_y.clamp(0, limit);
}
