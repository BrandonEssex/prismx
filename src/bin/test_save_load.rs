use prismx::node_tree::NodeTree;
use prismx::storage::{save::save_tree_to_file, load::load_tree_from_file};

use std::path::Path;

fn main() {
    let path = Path::new("test_output.gdz");

    // Build a test tree
    let mut tree = NodeTree::default();
    tree.create_child_node();
    tree.begin_editing_selected();

    // Save
    println!("Saving tree to {:?}", path);
    if let Err(e) = save_tree_to_file(&tree, path) {
        eprintln!("❌ Failed to save tree: {}", e);
        return;
    }

    // Load
    println!("Loading tree from {:?}", path);
    match load_tree_from_file(path) {
        Ok(loaded_tree) => {
            println!("✅ Loaded {} nodes:", loaded_tree.len());
            for node in loaded_tree.nodes {
                println!("- [{}] {}{}", node.id, node.label, if node.editing { " (editing)" } else { "" });
            }
        }
        Err(e) => {
            eprintln!("❌ Failed to load tree: {}", e);
        }
    }
}
