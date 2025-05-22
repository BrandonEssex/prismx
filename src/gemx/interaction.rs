use crate::state::AppState;
use crate::node::NodeID;

/// Drag a node by a delta in pixels.
/// When `snap_to_grid` is enabled on the state, the node will snap to
/// 20px increments after movement.
pub fn drag_node(state: &mut AppState, node_id: NodeID, dx: i16, dy: i16) {
    state.drag_node_position(node_id, dx, dy);
}

/// Toggle the global snap-to-grid option.
/// Bound to Ctrl+G via input hotkeys.
pub fn toggle_snap(state: &mut AppState) {
    state.toggle_snap_to_grid();
}
