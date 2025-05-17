use std::fs;

pub fn load_all_plugins() -> Result<(), Box<dyn std::error::Error>> {
    let plugins = vec!["gemx", "dashboard", "mindtrace"];
    for plugin in plugins {
        let path = format!("plugins/{}/lineage.trace.json", plugin);
        match fs::read_to_string(&path) {
            Ok(lineage) => println!("[PLUGIN] Loaded: {} → {}", plugin, lineage.trim()),
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
                println!("[PLUGIN] {} missing lineage file. Skipping.", plugin);
            }
            Err(e) => return Err(Box::new(e)),
        }
    }
    Ok(())
}
