use std::fs;

pub fn start_sync_if_enabled() {
    match fs::read_to_string("federation/sync_status.json") {
        Ok(data) => println!("[FEDERATION] Synced: {}", data.trim()),
        Err(_) => println!("[FEDERATION] No sync status found."),
    }
}
