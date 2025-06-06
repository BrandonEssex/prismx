use super::nodes::MindmapNode;
use std::fs;
use std::path::Path;

pub fn load() {
    let path = Path::new("snapshots/mindmap.json");

    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).ok();
    }

    match fs::read_to_string(&path) {
        Ok(data) => {
            if let Ok(root) = serde_json::from_str::<MindmapNode>(&data) {
                tracing::info!("[GEMX] Loaded mindmap: {}", root.title);
            } else {
                tracing::warn!("[GEMX] Invalid mindmap format.");
            }
        }
        Err(_) => {
            tracing::info!("[GEMX] No mindmap found. Creating default.");
            let root = MindmapNode::new("root", "Welcome to PrismX");
            let json = serde_json::to_string_pretty(&root).unwrap();
            let _ = fs::write(&path, json);
        }
    }
}
