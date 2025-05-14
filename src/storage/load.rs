use crate::node_tree::{Node, NodeTree};
use std::fs;
use std::path::Path;
use serde::Deserialize;

#[derive(Deserialize)]
struct SerializableNode {
    id: usize,
    label: String,
    editing: bool,
}

#[derive(Deserialize)]
struct SerializableTree {
    nodes: Vec<SerializableNode>,
}

pub fn load_tree_from_file(path: &Path) -> std::io::Result<NodeTree> {
    let content = fs::read_to_string(path)?;
    let parsed: SerializableTree = serde_json::from_str(&content)?;

    let mut tree = NodeTree {
        nodes: Vec::new(),
        selected_index: 0,
        next_id: 0,
    };

    for node in parsed.nodes {
        tree.next_id = tree.next_id.max(node.id + 1);
        tree.nodes.push(Node {
            id: node.id,
            label: node.label,
            editing: node.editing,
            children: Vec::new(), // placeholder until child serialization is added
        });
    }

    Ok(tree)
}
