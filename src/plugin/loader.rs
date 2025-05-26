use libloading::Library;
use std::path::{Path, PathBuf};

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

/// Search the given directory for `.so` or `.dylib` files and attempt to load
/// them using `libloading`. Any successfully opened libraries are returned.
pub fn discover_plugins(dir: &Path) -> Vec<LoadedPlugin> {
    let mut plugins = Vec::new();
    if let Ok(entries) = std::fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if matches!(path.extension().and_then(|e| e.to_str()), Some("so") | Some("dylib")) {
                match unsafe { Library::new(&path) } {
                    Ok(lib) => {
                        tracing::info!("[PLUGIN] discovered {}", path.display());
                        plugins.push(LoadedPlugin { path, lib });
                    }
                    Err(err) => {
                        tracing::warn!(
                            "[PLUGIN] failed to load {}: {}",
                            path.display(),
                            err
                        );
                    }
                }
            }
        }
    }
    plugins
}
