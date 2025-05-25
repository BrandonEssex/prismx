use crate::io::fs;

pub fn start_sync_if_enabled() {
    match fs::read_to_string("federation/sync_status.json") {
        Ok(data) => println!("[FEDERATION] Sync active: {}", data.trim()),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
            println!("[FEDERATION] No sync status file. Skipping sync.");
        }
        Err(e) => eprintln!("[FEDERATION] Error: {}", e),
    }
}
