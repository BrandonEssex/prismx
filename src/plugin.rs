use std::fs;

pub fn load_all_plugins() -> Result<(), Box<dyn std::error::Error>> {
    let plugins = vec!["gemx", "dashboard", "mindtrace"];
    for plugin in plugins {
        let manifest_path = format!("plugins/{}/lineage.trace.json", plugin);
        let lineage = fs::read_to_string(manifest_path)?;
        println!("[PLUGIN] Loaded: {} → {}", plugin, lineage.trim());
    }
    Ok(())
}
