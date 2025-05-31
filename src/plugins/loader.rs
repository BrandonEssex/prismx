use libloading::{Library, Symbol};
use std::path::{Path, PathBuf};

use super::registry;

pub trait PluginEntry {
    fn name(&self) -> &'static str;
    fn init(&mut self);
    fn tick(&mut self);
}

/// Represents a discovered plugin library.
pub struct LoadedPlugin {
    pub path: PathBuf,
    #[allow(dead_code)]
    lib: Library,
}

/// Attempt to load a single plugin library and invoke its `register` function.
pub fn load_plugin(path: &Path) -> Option<LoadedPlugin> {
    tracing::debug!("[PLUGIN] attempting {}", path.display());
    unsafe {
        match Library::new(path) {
            Ok(lib) => {
                let symbol: Result<Symbol<unsafe extern "C" fn()>, _> = lib.get(b"register");
                match symbol {
                    Ok(register) => {
                        tracing::info!("[PLUGIN] loaded {}", path.display());
                        register();
                        Some(LoadedPlugin { path: path.to_path_buf(), lib })
                    }
                    Err(err) => {
                        tracing::error!(
                            "[PLUGIN] missing register() in {}: {}",
                            path.display(),
                            err
                        );
                        None
                    }
                }
            }
            Err(err) => {
                tracing::error!("[PLUGIN] failed to load {}: {}", path.display(), err);
                None
            }
        }
    }
}

/// Search the given directory for `.so` or `.dylib` files and attempt to load
/// them using `libloading`. Any successfully opened libraries are returned.
pub fn discover_plugins(dir: &Path) -> Vec<LoadedPlugin> {
    tracing::debug!("[PLUGIN] scanning {}", dir.display());
    let mut plugins = Vec::new();
    if let Ok(entries) = std::fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if matches!(path.extension().and_then(|e| e.to_str()), Some("so") | Some("dylib")) {
                tracing::debug!("[PLUGIN] loading {}", path.display());
                if let Some(plugin) = load_plugin(&path) {
                    plugins.push(plugin);
                }
            }
        }
    }
    plugins
}

/// Load plugins defined in `manifest.json` within the given directory.
pub fn load_manifest_plugins(dir: &Path) -> Vec<LoadedPlugin> {
    let manifest_path = dir.join("manifest.json");
    let entries = registry::load_manifest(&manifest_path);
    let mut plugins = Vec::new();
    for entry in entries {
        let path = dir.join(&entry.entry);
        if path.exists() {
            if let Some(plugin) = load_plugin(&path) {
                plugins.push(plugin);
            }
        } else {
            tracing::warn!("[PLUGIN] missing entry {}", path.display());
        }
    }
    plugins
}
