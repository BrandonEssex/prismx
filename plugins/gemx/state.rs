use super::nodes::MindmapNode;
use std::fs;

pub fn load() {
    match fs::read_to_string("snapshots/mindmap.json") {
        Ok(data) => {
            if let Ok(root): Result<MindmapNode, _> = serde_json::from_str(&data) {
                println!("[GEMX] Loaded mindmap: {}", root.title);
            } else {
                println!("[GEMX] Invalid mindmap format.");
            }
        }
        Err(_) => println!("[GEMX] No mindmap found. Starting fresh."),
    }
}
