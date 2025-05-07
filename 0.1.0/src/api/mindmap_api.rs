use crate::storage::json_store;
use crate::tui::mindmap::{MindmapState, Position};
use crate::util::errors::MindmapError;
use log::{debug, error, info};
use tokio::sync::Mutex;
use lazy_static::lazy_static;

lazy_static! {
    static ref STATE: Mutex<MindmapState> = Mutex::new(MindmapState::new());
}

pub async fn create_node(content: &str, x: f64, y: f64) -> Result<u64, MindmapError> {
    let mut state = STATE.lock().await;
    let position = Position { x, y };
    let node_id = state.mindmap.add_node(content, position);
    debug!("Created node {} at position {:?}", node_id, position);
    Ok(node_id)
}

pub async fn delete_node(node_id: u64) -> Result<(), MindmapError> {
    let mut state = STATE.lock().await;
    if state.mindmap.remove_node(node_id) {
        debug!("Deleted node {}", node_id);
        Ok(())
    } else {
        error!("Attempted to delete nonexistent node {}", node_id);
        Err(MindmapError::NodeNotFound(node_id))
    }
}

pub async fn move_node(node_id: u64, x: f64, y: f64) -> Result<(), MindmapError> {
    let mut state = STATE.lock().await;
    let position = Position { x, y };
    if state.mindmap.move_node(node_id, position) {
        debug!("Moved node {} to {:?}", node_id, position);
        Ok(())
    } else {
        error!("Attempted to move nonexistent node {}", node_id);
        Err(MindmapError::NodeNotFound(node_id))
    }
}

pub async fn update_node_content(node_id: u64, new_content: &str) -> Result<(), MindmapError> {
    let mut state = STATE.lock().await;
    if let Some(node) = state.mindmap.nodes.get_mut(&node_id) {
        node.content = new_content.to_string();
        debug!("Updated node {} content to '{}'", node_id, new_content);
        Ok(())
    } else {
        error!("Attempted to update content of nonexistent node {}", node_id);
        Err(MindmapError::NodeNotFound(node_id))
    }
}

pub async fn get_node(node_id: u64) -> Result<crate::tui::mindmap::Node, MindmapError> {
    let state = STATE.lock().await;
    state
        .mindmap
        .nodes
        .get(&node_id)
        .cloned()
        .ok_or_else(|| {
            error!("Attempted to get nonexistent node {}", node_id);
            MindmapError::NodeNotFound(node_id)
        })
}

pub async fn save_mindmap_async(path: Option<&str>) -> Result<(), MindmapError> {
    let state = STATE.lock().await;
    json_store::save_mindmap(&state.mindmap, path).await?;
    info!("Mindmap saved asynchronously.");
    Ok(())
}

pub async fn load_mindmap_async(path: Option<&str>) -> Result<(), MindmapError> {
    let mut state = STATE.lock().await;
    state.mindmap = json_store::load_mindmap(path).await?;
    info!("Mindmap loaded asynchronously.");
    Ok(())
}