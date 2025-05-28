use alloc::collections::HashMap;
use crate::node::NodeID;
use crate::state::AppState;
use crate::layout::{LayoutRole, CHILD_SPACING_Y};

/// Recalculate parent/child relationships based on node positions.
/// Nodes directly beneath another (±1 cell) become children of that node.
/// Nodes on the same row become siblings if that row already has a parent.
/// Otherwise the node is considered free/root.
pub fn recalculate_roles(state: &mut AppState) {
    state.clear_fallback_promotions();
    state.layout_roles.clear();

    let ids: Vec<NodeID> = state.nodes.keys().copied().collect();

    // Clear current structure
    for node in state.nodes.values_mut() {
        node.children.clear();
        node.parent = None;
    }
    state.root_nodes.clear();

    // Pass 1: detect direct parent above
    let mut new_parents: HashMap<NodeID, Option<NodeID>> = HashMap::new();
    for &id in &ids {
        let (x, y) = {
            let n = &state.nodes[&id];
            (n.x, n.y)
        };
        let mut parent_id = None;
        for &other in &ids {
            if other == id {
                continue;
            }
            let op = &state.nodes[&other];
            if y > op.y && (y - op.y) <= CHILD_SPACING_Y + 1 && (x - op.x).abs() <= 1 {
                parent_id = Some(other);
                break;
            }
        }
        new_parents.insert(id, parent_id);
    }

    // Pass 2: siblings on same row inherit existing parent
    for &id in &ids {
        if new_parents[&id].is_some() {
            continue;
        }
        let y = state.nodes[&id].y;
        for &other in &ids {
            if other == id {
                continue;
            }
            if (state.nodes[&other].y - y).abs() <= 1 {
                if let Some(pid) = new_parents[&other] {
                    new_parents.insert(id, Some(pid));
                    break;
                }
            }
        }
    }

    // Apply structure and lock child positions
    for &id in &ids {
        if let Some(parent_id) = new_parents[&id] {
            if parent_id == id {
                continue;
            }
            crate::log_debug!(state, "Assigning parent {:?} \u{2192} {}", parent_id, id);
            let (px, py) = {
                let p = &state.nodes[&parent_id];
                (p.x, p.y)
            };
            if let Some(node) = state.nodes.get_mut(&id) {
                node.parent = Some(parent_id);
                node.x = px;
                node.y = py + CHILD_SPACING_Y;
            }
            if let Some(parent) = state.nodes.get_mut(&parent_id) {
                parent.children.push(id);
            }
        } else {
            if let Some(node) = state.nodes.get_mut(&id) {
                node.parent = None;
            }
            state.root_nodes.push(id);
        }
    }

    // Deduplicate lists
    state.root_nodes.sort_unstable();
    state.root_nodes.dedup();
    for node in state.nodes.values_mut() {
        node.children.sort_unstable();
        node.children.dedup();
    }

    for &id in &state.root_nodes {
        state.layout_roles.insert(id, LayoutRole::Root);
    }
}
