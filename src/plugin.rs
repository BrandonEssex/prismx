use std::fs;

pub fn load_all_plugins() -> Result<(), Box<dyn std::error::Error>> {
    for plugin in ["mindmaps", "gemdrop"] {
        let manifest_path = format!("plugins/{}/lineage.trace.json", plugin);
        let lineage = fs::read_to_string(manifest_path)?;
        println!("[PLUGIN] Loaded lineage for {}: {}", plugin, lineage);
    }
    Ok(())
}
