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

/// Attempt to load a single plugin library and validate that it exposes the
/// required `prismx_plugin_entry` symbol.
pub fn load_plugin(path: &str) -> Option<LoadedPlugin> {
    tracing::debug!("[PLUGIN] attempting {}", path);
    unsafe {
        match Library::new(path) {
            Ok(lib) => {
                let has_entry = lib.get::<libloading::Symbol<unsafe extern "C" fn()>>(b"prismx_plugin_entry").is_ok();
                if has_entry {
                    tracing::info!("[PLUGIN] loaded {}", path);
                    Some(LoadedPlugin { path: PathBuf::from(path), lib })
                } else {
                    tracing::error!("[PLUGIN] missing entry symbol in {}", path);
                    None
                }
            }
            Err(err) => {
                tracing::error!("[PLUGIN] failed to load {}: {}", path, err);
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
