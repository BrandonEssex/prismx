use crate::node::{NodeID, NodeMap};

/// Move `node_id` under `target` or promote to root when `target` is `None`.
pub fn reparent(nodes: &mut NodeMap, roots: &mut Vec<NodeID>, node_id: NodeID, target: Option<NodeID>) {
    if let Some(tid) = target {
        if tid == node_id {
            tracing::warn!("❌ Invalid reparent: node cannot be its own parent");
            return;
        }
        let mut cur = Some(tid);
        while let Some(cid) = cur {
            if cid == node_id {
                tracing::warn!("❌ Invalid reparent: cycle detected");
                return;
            }
            cur = nodes.get(&cid).and_then(|n| n.parent);
        }
    }

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

/// Promote `node_id` one level up in the hierarchy.
pub fn promote(nodes: &mut NodeMap, roots: &mut Vec<NodeID>, node_id: NodeID) {
    if let Some(parent_id) = nodes.get(&node_id).and_then(|n| n.parent) {
        let grand = nodes.get(&parent_id).and_then(|p| p.parent);
        reparent(nodes, roots, node_id, grand);
    }
}

/// Demote `node_id` under its previous sibling if possible.
pub fn demote_prev_sibling(
    nodes: &mut NodeMap,
    roots: &mut Vec<NodeID>,
    node_id: NodeID,
) {
    if let Some(parent_id) = nodes.get(&node_id).and_then(|n| n.parent) {
        if let Some(parent) = nodes.get(&parent_id) {
            if let Some(pos) = parent.children.iter().position(|&c| c == node_id) {
                if pos > 0 {
                    let prev = parent.children[pos - 1];
                    reparent(nodes, roots, node_id, Some(prev));
                }
            }
        }
    }
}

/// Return the parent ID for `node_id`.
pub fn parent_id(nodes: &NodeMap, node_id: NodeID) -> Option<NodeID> {
    nodes.get(&node_id).and_then(|n| n.parent)
}

/// Determine how deep a node is in the hierarchy.
pub fn node_depth(nodes: &NodeMap, node_id: NodeID) -> usize {
    let mut depth = 0usize;
    let mut current = nodes.get(&node_id).and_then(|n| n.parent);
    let mut visited = std::collections::HashSet::new();
    while let Some(pid) = current {
        if !visited.insert(pid) {
            break;
        }
        depth += 1;
        current = nodes.get(&pid).and_then(|n| n.parent);
        if depth > nodes.len() {
            break;
        }
    }
    depth
}

/// Merge `src` node into `tgt` when they share the same depth.
/// Children from `src` are reparented to `tgt` and `src` is removed.
pub fn merge_nodes(
    nodes: &mut NodeMap,
    roots: &mut Vec<NodeID>,
    src: NodeID,
    tgt: NodeID,
) {
    if src == tgt {
        return;
    }
    if node_depth(nodes, src) != node_depth(nodes, tgt) {
        return;
    }

    // Detach src from its parent or root list
    if let Some(pid) = nodes.get(&src).and_then(|n| n.parent) {
        if let Some(parent) = nodes.get_mut(&pid) {
            parent.children.retain(|&c| c != src);
        }
    } else {
        roots.retain(|&r| r != src);
    }

    if let Some(mut s) = nodes.remove(&src) {
        let child_ids = s.children.clone();
        if let Some(mut t) = nodes.remove(&tgt) {
            if !t.label.is_empty() && !s.label.is_empty() {
                t.label.push(' ');
            }
            t.label.push_str(&s.label);
            for child in child_ids {
                if let Some(c) = nodes.get_mut(&child) {
                    c.parent = Some(tgt);
                }
                if !t.children.contains(&child) {
                    t.children.push(child);
                }
            }
            nodes.insert(tgt, t);
        } else {
            // put source back if target missing
            nodes.insert(src, s);
        }
    }
}
