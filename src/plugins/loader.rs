use libloading::{Library, Symbol};
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

/// Safely open a dynamic library. This wraps the unsafe `Library::new` call
/// behind a safe function boundary.
fn open_library(path: &Path) -> Result<Library, libloading::Error> {
    // SAFETY: Loading a library is unsafe because the caller must uphold that
    // the loaded library is a valid shared object for the duration of the
    // `Library` handle. We only expose a safe API and keep the `Library`
    // owned by `LoadedPlugin`, ensuring it lives as long as needed.
    unsafe { Library::new(path) }
}

/// Attempt to load a single plugin library and validate that it exposes the
/// required `prismx_plugin_entry` symbol.
pub fn load_plugin(path: &Path) -> Option<LoadedPlugin> {
    tracing::debug!("[PLUGIN] attempting {}", path.display());
    match open_library(path) {
        Ok(lib) => {
            // SAFETY: the symbol type matches the expected `register` signature
            // provided by plugins.
            let register: Result<Symbol<unsafe extern "C" fn()>, _> = unsafe { lib.get(b"register") };
            match register {
                Ok(func) => {
                    unsafe { func() };
                    tracing::info!("[PLUGIN] loaded {}", path.display());
                    Some(LoadedPlugin { path: path.to_path_buf(), lib })
                }
                Err(err) => {
                    tracing::error!("[PLUGIN] missing register() in {}: {}", path.display(), err);
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
                match open_library(&path) {
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

/// Load all plugins within a directory and invoke their `register` functions.
/// Returns a vector of successfully loaded plugins.
pub fn load_plugins(dir: &Path) -> Vec<LoadedPlugin> {
    tracing::debug!("[PLUGIN] loading from {}", dir.display());
    let mut plugins = Vec::new();
    if let Ok(entries) = std::fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if matches!(path.extension().and_then(|e| e.to_str()), Some("so") | Some("dylib")) {
                if let Some(plugin) = load_plugin(&path) {
                    plugins.push(plugin);
                }
            }
        }
    }
    plugins
}
