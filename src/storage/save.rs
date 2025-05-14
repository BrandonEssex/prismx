use crate::node_tree::NodeTree;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use serde::Serialize;

#[derive(Serialize)]
struct SerializableNode {
    id: usize,
    label: String,
    editing: bool,
}

#[derive(Serialize)]
struct SerializableTree {
    nodes: Vec<SerializableNode>,
}

pub fn save_tree_to_file(tree: &NodeTree, path: &Path) -> std::io::Result<()> {
    let data = SerializableTree {
        nodes: tree.nodes.iter().map(|n| SerializableNode {
            id: n.id,
            label: n.label.clone(),
            editing: n.editing,
        }).collect(),
    };

    let json = serde_json::to_string_pretty(&data)?;
    let mut file = File::create(path)?;
    file.write_all(json.as_bytes())?;
    Ok(())
}
