use crate::tui::mindmap::Mindmap;
use crate::util::errors::MindmapError;
use tokio::fs;
use tokio::io::AsyncWriteExt;
use log::{error, info, warn};

const DEFAULT_PATH: &str = "data/mindmaps.json";
const BACKUP_PATH: &str = "data/mindmaps_backup.json";

pub async fn save_mindmap(mindmap: &Mindmap, path: Option<&str>) -> Result<(), MindmapError> {
    let serialized = serde_json::to_string_pretty(mindmap)
        .map_err(|e| MindmapError::PersistenceFailure(e.to_string()))?;

    let path = path.unwrap_or(DEFAULT_PATH);
    let mut file = fs::File::create(path)
        .await
        .map_err(|e| MindmapError::PersistenceFailure(e.to_string()))?;

    file.write_all(serialized.as_bytes())
        .await
        .map_err(|e| MindmapError::PersistenceFailure(e.to_string()))?;

    info!("Mindmap successfully saved to '{}'.", path);
    Ok(())
}

pub async fn load_mindmap(path: Option<&str>) -> Result<Mindmap, MindmapError> {
    let path = path.unwrap_or(DEFAULT_PATH);

    match fs::read_to_string(path).await {
        Ok(content) => match serde_json::from_str::<Mindmap>(&content) {
            Ok(mindmap) => {
                info!("Mindmap successfully loaded from '{}'.", path);
                Ok(mindmap)
            }
            Err(parse_err) => {
                error!("JSON parse error: {}. Initiating recovery...", parse_err);
                perform_fault_recovery(&content).await?;
                Err(MindmapError::JsonParseError(parse_err.to_string()))
            }
        },
        Err(read_err) => {
            error!("Failed to read mindmap file '{}': {}", path, read_err);
            Err(MindmapError::PersistenceFailure(read_err.to_string()))
        }
    }
}

async fn perform_fault_recovery(corrupted_content: &str) -> Result<(), MindmapError> {
    warn!("Performing fault recovery: backing up corrupted mindmap.");
    let mut backup_file = fs::File::create(BACKUP_PATH)
        .await
        .map_err(|e| MindmapError::PersistenceFailure(e.to_string()))?;

    backup_file
        .write_all(corrupted_content.as_bytes())
        .await
        .map_err(|e| MindmapError::PersistenceFailure(e.to_string()))?;

    warn!("Backup of corrupted mindmap created at '{}'.", BACKUP_PATH);
    Ok(())
}