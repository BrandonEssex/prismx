use std::collections::{HashMap, HashSet};
use crate::state::AppState;
use crate::node::NodeID;
use crate::layout::{Coords, LayoutRole, GEMX_HEADER_HEIGHT};

/// Promote unreachable nodes to root positions when auto-arrange is enabled.
/// This helps recover nodes that would otherwise be lost off-screen.
#[allow(clippy::too_many_arguments)]
pub fn promote_unreachable(
    state: &mut AppState,
    reachable_ids: &HashSet<NodeID>,
    drawn_at: &mut HashMap<NodeID, Coords>,
    node_roles: &mut HashMap<NodeID, LayoutRole>,
    area_height: i16,
) {
    let node_ids: Vec<NodeID> = state.nodes.keys().copied().collect();
    for id in node_ids {
        if state.fallback_this_frame {
            continue;
        }
        let node = match state.nodes.get(&id) {
            Some(n) => n,
            None => continue,
        };
        if state.root_nodes.contains(&id)
            || drawn_at.contains_key(&id)
            || reachable_ids.contains(&id)
            || state.fallback_promoted_this_session.contains(&id)
        {
            continue;
        }
        if node.children.is_empty() {
            continue;
        }

        state.root_nodes.push(id);
        state.root_nodes.sort_unstable();
        state.root_nodes.dedup();
        state.fallback_this_frame = true;
        state.fallback_promoted_this_session.insert(id);

        if let Some(n) = state.nodes.get_mut(&id) {
            if n.x == 0 && n.y == 0 {
                n.x = state.fallback_next_x;
                n.y = state.fallback_next_y;
                state.fallback_next_y += 3;
                if state.fallback_next_y > area_height - 4 {
                    state.fallback_next_y = GEMX_HEADER_HEIGHT + 2;
                    state.fallback_next_x += 20;
                }
                if state.debug_input_mode {
                    eprintln!("\u{1F4D0} Placed Node {} at x={}, y={}", id, n.x, n.y);
                }
            }
            drawn_at.insert(id, Coords { x: n.x, y: n.y });
            node_roles.insert(id, LayoutRole::Root);
        } else {
            eprintln!("❌ Fallback failed: Node {} not found.", id);
        }

        crate::log_debug!(state, "\u{26a0} Promoted Node {} to root (label-safe)", id);
        break;
    }
}
