use std::fs;
use std::path::Path;
use serde::{Serialize, Deserialize};
use crate::node::{NodeMap, NodeID};

#[derive(Clone, Serialize, Deserialize, Default)]
pub struct WorkspaceData {
    pub nodes: NodeMap,
    pub root_nodes: Vec<NodeID>,
}

pub fn save_workspace(data: &WorkspaceData) -> std::io::Result<()> {
    let path = Path::new("./data/workspace.json");
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    let json = serde_json::to_string_pretty(data).map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
    fs::write(path, json)
}

pub fn load_workspace() -> std::io::Result<WorkspaceData> {
    let path = Path::new("./data/workspace.json");
    let data = fs::read_to_string(path)?;
    let ws: WorkspaceData = serde_json::from_str(&data).map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
    Ok(ws)
}
