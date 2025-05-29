use crate::node::{NodeID, NodeMap};

/// Move `node_id` under `target` or promote to root when `target` is `None`.
pub fn reparent(nodes: &mut NodeMap, roots: &mut Vec<NodeID>, node_id: NodeID, target: Option<NodeID>) {
    // Detach from current parent or root list
    if let Some(pid) = nodes.get(&node_id).and_then(|n| n.parent) {
        if let Some(parent) = nodes.get_mut(&pid) {
            parent.children.retain(|&c| c != node_id);
        }
    } else {
        roots.retain(|&r| r != node_id);
    }

    match target {
        Some(tid) => {
            nodes.get_mut(&node_id).map(|n| n.parent = Some(tid));
            if let Some(parent) = nodes.get_mut(&tid) {
                if !parent.children.contains(&node_id) {
                    parent.children.push(node_id);
                }
            }
        }
        None => {
            nodes.get_mut(&node_id).map(|n| n.parent = None);
            if !roots.contains(&node_id) {
                roots.push(node_id);
            }
        }
    }
}

/// Ensure all orphaned nodes become roots.
pub fn adopt_orphans(nodes: &mut NodeMap, roots: &mut Vec<NodeID>) {
    for id in nodes.keys().copied().collect::<Vec<_>>() {
        if nodes[&id].parent.is_none() && !roots.contains(&id) {
            roots.push(id);
        }
    }
    roots.sort_unstable();
    roots.dedup();
}
