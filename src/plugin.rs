use std::fs;

#[derive(Debug)]
pub struct Plugin {
    pub name: String,
    pub path: String,
}

impl Plugin {
    pub fn load_manifest(path: &str) -> Result<String, Box<dyn std::error::Error>> {
        let contents = fs::read_to_string(path)?;
        Ok(contents)
    }

    pub fn list_plugins() -> Vec<Plugin> {
        let mut plugins = Vec::new();
        if let Ok(entries) = fs::read_dir("plugin/") {
            for entry in entries.flatten() {
                if let Ok(path) = entry.path().into_os_string().into_string() {
                    if path.ends_with(".toml") {
                        plugins.push(Plugin {
                            name: entry.file_name().into_string().unwrap_or_default(),
                            path,
                        });
                    }
                }
            }
        }
        plugins
    }
}
