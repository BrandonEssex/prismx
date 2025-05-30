use crate::state::AppState;
use crate::node::NodeID;
use super::viewport;
pub use crate::layout::engine::sibling_offset;

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

/// Determine dynamic child spacing based on total depth.
pub fn clamp_child_spacing(state: &AppState, roots: &[NodeID], max_h: i16) -> i16 {
    use crate::layout::{CHILD_SPACING_Y, MIN_CHILD_SPACING_Y, subtree_depth};

    let mut depth = 1i16;
    for r in roots {
        depth = depth.max(subtree_depth(&state.nodes, *r));
    }
    let required = depth * CHILD_SPACING_Y;
    if max_h > 0 && required > max_h {
        let ratio = max_h as f32 / required as f32;
        ((CHILD_SPACING_Y as f32 * ratio).floor() as i16).max(MIN_CHILD_SPACING_Y)
    } else {
        CHILD_SPACING_Y
    }
}
