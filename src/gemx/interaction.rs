use crate::state::AppState;
use crate::node::NodeID;

/// Toggle snap-to-grid mode
pub fn toggle_snap(state: &mut AppState) {
    state.toggle_snap_grid();
}

/// Drag currently selected node by delta values.
/// Children move together implicitly because coordinates are stored per node.
pub fn drag_node(state: &mut AppState, dx: i16, dy: i16) {
    if let Some(id) = state.selected {
        if let Some(node) = state.nodes.get_mut(&id) {
            node.x += dx;
            node.y += dy;
            if state.snap_to_grid {
                node.x = snap_value(node.x);
                node.y = snap_value(node.y);
            }
        }
    }
}

fn snap_value(v: i16) -> i16 {
    ((v + 10) / 20) * 20
}
