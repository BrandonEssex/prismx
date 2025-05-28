use std::fs;

pub fn start_sync_if_enabled() {
    match fs::read_to_string("federation/sync_status.json") {
        Ok(data) => tracing::info!("[FEDERATION] Sync active: {}", data.trim()),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
            tracing::info!("[FEDERATION] No sync status file. Skipping sync.");
        }
        Err(e) => tracing::warn!("[FEDERATION] Error: {}", e),
    }
}
