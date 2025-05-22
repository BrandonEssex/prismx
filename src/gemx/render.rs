use std::collections::HashMap;

use crate::{layout, node::NodeID, state::AppState};
use layout::Coords;

/// Calculate drawing coordinates for nodes.
/// When auto_arrange is enabled, uses the tree layout. Otherwise,
/// returns the manually stored x/y positions on each node.
pub fn calculate_positions(state: &AppState, root: NodeID) -> HashMap<NodeID, Coords> {
    if state.auto_arrange {
        layout::layout_nodes(&state.nodes, root, 2, 1)
    } else {
        state
            .nodes
            .iter()
            .map(|(&id, n)| {
                (
                    id,
                    Coords {
                        x: n.x as u16,
                        y: n.y as u16,
                    },
                )
            })
            .collect()
    }
}
