#![cfg(feature = "std")]

use alloc::collections::{BTreeMap, BTreeSet};
use alloc::vec::Vec;
use crate::state::AppState;
use crate::core::node::NodeID;
use crate::core::layout::{Coords, LayoutRole, GEMX_HEADER_HEIGHT};

#[allow(clippy::too_many_arguments)]
pub fn promote_unreachable(
    state: &mut AppState,
    reachable_ids: &BTreeSet<NodeID>,
    drawn_at: &mut BTreeMap<NodeID, Coords>,
    node_roles: &mut BTreeMap<NodeID, LayoutRole>,
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

        let filled: BTreeSet<(i16, i16)> =
            state.nodes.values().map(|n| (n.x, n.y)).collect();

        if let Some(n) = state.nodes.get_mut(&id) {
            if n.x == 0 && n.y == 0 {
                let step_x = 20;
                let step_y = 3;
                let base_y = GEMX_HEADER_HEIGHT + 2;
                let max_y = area_height - 4;
                let mut x = state.fallback_next_x;
                let mut y = state.fallback_next_y;

                while filled.contains(&(x, y)) {
                    if state.debug_input_mode {
                        tracing::debug!("↪ collision at {},{}", x, y);
                    }
                    y += step_y;
                    if y > max_y {
                        y = base_y;
                        x += step_x;
                    }
                }

                n.x = x;
                n.y = y;
                state.fallback_next_x = x;
                state.fallback_next_y = y + step_y;

                if state.debug_input_mode {
                    tracing::debug!("\u{1F4D0} Placed Node {} at x={}, y={}", id, n.x, n.y);
                }
            }
            drawn_at.insert(id, Coords { x: n.x, y: n.y });
            node_roles.insert(id, LayoutRole::Root);
        } else {
            tracing::error!("❌ Fallback failed: Node {} not found.", id);
        }

        crate::log_debug!(state, "\u{26a0} Promoted Node {} to root (label-safe)", id);
        break;
    }
}
