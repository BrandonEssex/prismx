use std::collections::HashSet;
use std::fs;

pub fn load_plugin_bundle(path: &str) -> Option<HashSet<String>> {
    let data = fs::read_to_string(path).ok()?;
    let mut active = HashSet::new();

    for line in data.lines() {
        if let Some(plugin_id) = line.strip_prefix("enable:") {
            active.insert(plugin_id.trim().to_string());
        }
    }

    Some(active)
}
