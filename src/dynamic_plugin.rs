use crate::state::AppState;
use libloading::{Library, Symbol};
use std::fs;
use std::path::PathBuf;

pub trait PrismPlugin {
    fn register(state: &mut AppState);
}

pub struct DynamicPlugin {
    _lib: Library,
}

pub fn load_dynamic_plugins(state: &mut AppState) -> Vec<DynamicPlugin> {
    let mut plugins = Vec::new();
    let mut dir = match dirs::home_dir() {
        Some(p) => p,
        None => PathBuf::from("."),
    };
    dir.push(".prismx/plugins");
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
                if ext == "so" || ext == "dylib" {
                    match unsafe { Library::new(&path) } {
                        Ok(lib) => {
                            let reg: libloading::Result<Symbol<unsafe extern "C" fn(&mut AppState)>> = unsafe { lib.get(b"prismx_register") };
                            match reg {
                                Ok(func) => {
                                    unsafe { func(state) };
                                    crate::log_info!("Loaded plugin: {:?}", path);
                                    plugins.push(DynamicPlugin { _lib: lib });
                                }
                                Err(e) => {
                                    crate::log_warn!("Registration symbol missing in {:?}: {}", path, e);
                                }
                            }
                        }
                        Err(e) => {
                            crate::log_warn!("Failed to load plugin {:?}: {}", path, e);
                        }
                    }
                }
            }
        }
    }
    plugins
}
