// src/node_tree.rs

use crate::node::Node;
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Default)]
pub struct NodeTree {
    pub nodes: HashMap<Uuid, Node>,
    pub root_ids: Vec<Uuid>,
}

impl NodeTree {
    pub fn add_node(&mut self, node: Node) {
        if let Some(parent_id) = node.parent {
            if let Some(parent) = self.nodes.get_mut(&parent_id) {
                parent.children.push(node.id);
            }
        } else {
            self.root_ids.push(node.id);
        }
        self.nodes.insert(node.id, node);
    }

    pub fn get_node(&self, id: &Uuid) -> Option<&Node> {
        self.nodes.get(id)
    }

    pub fn get_children(&self, id: &Uuid) -> Vec<&Node> {
        self.nodes.get(id)
            .map(|n| n.children.iter().filter_map(|cid| self.nodes.get(cid)).collect())
            .unwrap_or_default()
    }
}