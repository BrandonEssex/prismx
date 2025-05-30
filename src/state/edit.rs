use super::core::AppState;
use crate::node::{Node, NodeID};
use crate::layout::{SIBLING_SPACING_X, CHILD_SPACING_Y, GEMX_HEADER_HEIGHT};

impl AppState {
    pub fn add_child(&mut self) { self.add_child_node(); }
    pub fn add_sibling(&mut self) { self.add_sibling_node(); }

    pub fn add_child_node(&mut self) {
        let Some(parent_id) = self.selected else { return };
        if !self.can_insert_node() {
            self.status_message = "Cannot insert: empty node".into();
            self.status_message_last_updated = Some(std::time::Instant::now());
            return;
        }
        if !self.nodes.contains_key(&parent_id) {
            return;
        }

        let new_id = self.next_node_id;
        self.next_node_id += 1;

        if parent_id == new_id {
            tracing::warn!("❌ Invalid insert: node cannot parent itself.");
            return;
        }

        if let Some(parent) = self.nodes.get(&parent_id) {
            if parent.children.contains(&parent_id) {
                tracing::warn!("❌ Cycle detected: parent already linked to self.");
                return;
            }
        }

        let label = self.next_label();
        let mut child = Node::new(new_id, &label, Some(parent_id));
        if self.auto_arrange {
            if let Some(parent) = self.nodes.get(&parent_id) {
                child.x = parent.x;
                // more generous spacing for readability
                child.y = parent.y + CHILD_SPACING_Y;
            }
        } else {
            let base_x = 6 + ((self.nodes.len() as i16) % 10) * SIBLING_SPACING_X;
            let base_y = GEMX_HEADER_HEIGHT + 2 + ((self.nodes.len() as i16) / 10) * CHILD_SPACING_Y;
            child.x = base_x;
            child.y = base_y;
        }

        self.nodes.insert(new_id, child);
        crate::log_debug!(self, "Inserted node {} → parent {:?}", new_id, parent_id);
        if let Some(parent) = self.nodes.get_mut(&parent_id) {
            parent.children.push(new_id);
        }

        if !self.root_nodes.contains(&parent_id) {
            self.root_nodes.push(parent_id);
            self.root_nodes.sort_unstable();
            self.root_nodes.dedup();
        }

        self.set_selected(Some(new_id));
        if !self.auto_arrange {
            self.ensure_grid_positions();
            crate::layout::roles::recalculate_roles(self);
        }
        if self.nodes.get(&new_id).and_then(|n| n.parent).is_none() {
            if let Some(n) = self.nodes.get_mut(&new_id) {
                n.parent = Some(parent_id);
            }
            if let Some(p) = self.nodes.get_mut(&parent_id) {
                p.children.push(new_id);
            }
        }
        self.ensure_valid_roots();
        self.audit_ancestry();
    }

    pub fn add_sibling_node(&mut self) {
        let selected_id = match self.selected {
            Some(id) if self.nodes.contains_key(&id) => id,
            _ => return,
        };
        if !self.can_insert_node() {
            self.status_message = "Cannot insert: empty node".into();
            self.status_message_last_updated = Some(std::time::Instant::now());
            return;
        }
        let parent_id = self.nodes.get(&selected_id).and_then(|n| n.parent);

        let new_id = self.next_node_id;
        self.next_node_id += 1;
        let label = self.next_label();
        let mut sibling = Node::new(new_id, &label, parent_id);

        if let Some(selected) = self.nodes.get(&selected_id) {
            sibling.x = selected.x + SIBLING_SPACING_X;
            sibling.y = selected.y;
        } else if !self.auto_arrange {
            sibling.x = (self.nodes.len() as i16 % 5) * SIBLING_SPACING_X;
            sibling.y = GEMX_HEADER_HEIGHT + 2;

            if sibling.x == 0 {
                sibling.x = ((self.nodes.len() as i16) % 5 + 1) * SIBLING_SPACING_X;
            }
        }

        if parent_id.is_none() {
            self.root_nodes.push(new_id);
            self.root_nodes.sort_unstable();
            self.root_nodes.dedup();
        } else if let Some(parent) = self.nodes.get_mut(&parent_id.unwrap()) {
            parent.children.push(new_id);
        }

        self.nodes.insert(new_id, sibling);
        crate::log_debug!(self, "Inserted node {} → parent {:?}", new_id, parent_id);
        self.set_selected(Some(new_id));

        if !self.auto_arrange {
            self.ensure_grid_positions();
            crate::layout::roles::recalculate_roles(self);
        }
        self.ensure_valid_roots();
        self.audit_node_graph();
        self.audit_ancestry();
    }

    pub fn delete_node(&mut self) {
        if let Some(target_id) = self.selected {
            let parent_id = self.nodes.get(&target_id).and_then(|n| n.parent);
            if let Some(pid) = parent_id {
                if let Some(parent) = self.nodes.get_mut(&pid) {
                    parent.children.retain(|&id| id != target_id);
                }
            } else {
                self.root_nodes.retain(|&id| id != target_id);
            }

            fn delete_recursive(map: &mut crate::node::NodeMap, id: NodeID) {
                if let Some(node) = map.remove(&id) {
                    for child in node.children {
                        delete_recursive(map, child);
                    }
                }
            }

            delete_recursive(&mut self.nodes, target_id);
            self.set_selected(parent_id.or_else(|| self.root_nodes.first().copied()));
        }
    }

    pub fn toggle_collapse(&mut self) {
        if let Some(node) = self.get_selected_node_mut() {
            node.collapsed = !node.collapsed;
        }
    }

    pub fn add_free_node(&mut self) {
        let new_id = self.next_node_id;
        self.next_node_id += 1;
        let label = self.next_label();
        let mut node = Node::new(new_id, &label, None);

        if !self.auto_arrange {
            node.x = (self.nodes.len() as i16 % 5) * SIBLING_SPACING_X;
            node.y = GEMX_HEADER_HEIGHT + 2;
        }

        self.nodes.insert(new_id, node);
        self.root_nodes.push(new_id);
        self.set_selected(Some(new_id));

        crate::layout::roles::recalculate_roles(self);
        self.ensure_valid_roots();
        self.audit_node_graph();
        self.audit_ancestry();
    }

    /// Promote the currently selected node one level up the hierarchy.
    ///
    /// If the node has a parent, it becomes a sibling of that parent. If the
    /// parent is a root node, the promoted node becomes a new root. When the
    /// node is already a root, this is a no-op.
    pub fn promote_selected_node(&mut self) {
        let Some(id) = self.selected else { return };
        let Some(parent_id) = self.nodes.get(&id).and_then(|n| n.parent) else {
            return;
        };

        // Detach from current parent
        if let Some(parent) = self.nodes.get_mut(&parent_id) {
            parent.children.retain(|&c| c != id);
        }

        // Determine new parent (grandparent)
        let new_parent = self.nodes.get(&parent_id).and_then(|p| p.parent);
        if let Some(node) = self.nodes.get_mut(&id) {
            node.parent = new_parent;
        }

        match new_parent {
            Some(pid) => {
                if let Some(pp) = self.nodes.get_mut(&pid) {
                    if !pp.children.contains(&id) {
                        pp.children.push(id);
                    }
                }
                self.root_nodes.retain(|&r| r != id);
            }
            None => {
                if !self.root_nodes.contains(&id) {
                    self.root_nodes.push(id);
                }
            }
        }

        self.ensure_valid_roots();
        self.audit_node_graph();
    }
}
