use std::fs;
use std::path::Path;
use std::io::{Read, Write};

use crate::mindmap_state::MindmapState;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct SavedMindmap {
    pub state: MindmapState,
}

pub fn save_to_file(state: &MindmapState, path: &Path) -> std::io::Result<()> {
    let wrapper = SavedMindmap { state: state.clone() };
    let json = serde_json::to_string_pretty(&wrapper)?;
    fs::write(path, json)?;
    Ok(())
}

pub fn load_from_file(path: &Path) -> std::io::Result<MindmapState> {
    let mut file = fs::File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let wrapper: SavedMindmap = serde_json::from_str(&contents)?;
    Ok(wrapper.state)
}