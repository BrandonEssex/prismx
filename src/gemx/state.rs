use super::nodes::MindmapNode;
use std::fs;
use std::path::Path;

pub fn load() {
    let path = Path::new("snapshots/mindmap.json");

    // Create snapshots directory if missing
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).ok();
    }

    // Load or initialize empty mindmap
    match fs::read_to_string(&path) {
        Ok(data) => {
            if let Ok(root): Result<MindmapNode, _> = serde_json::from_str(&data) {
                println!("[GEMX] Loaded mindmap: {}", root.title);
            } else {
                println!("[GEMX] Invalid mindmap format.");
            }
        }
        Err(_) => {
            println!("[GEMX] No mindmap found. Creating default.");
            let root = MindmapNode::new("root", "Welcome to PrismX");
            let _ = fs::write(&path, serde_json::to_string_pretty(&root).unwrap());
        }
    }
}
