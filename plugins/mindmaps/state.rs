use super::nodes::MindmapNode;
use std::fs;

pub fn load_mindmap_state() {
    match fs::read_to_string("snapshots/mindmap.json") {
        Ok(data) => {
            let root: MindmapNode = serde_json::from_str(&data).unwrap_or_else(|_| MindmapNode::new("root", "Untitled"));
            println!("[MINDMAPS] Loaded root node: {:?}", root.title);
        }
        Err(_) => {
            println!("[MINDMAPS] No mindmap.json found, starting fresh.");
        }
    }
}
