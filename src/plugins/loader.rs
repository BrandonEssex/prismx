use std::ffi::CStr;
use std::os::raw::c_char;
use std::path::{Path, PathBuf};
use std::sync::{Mutex, OnceLock};

use libloading::{Library, Symbol};

#[derive(Debug, Clone)]
pub struct PluginMetadata {
    pub name: String,
    pub version: String,
    pub path: PathBuf,
}

#[repr(C)]
pub struct RawPluginMetadata {
    name: *const c_char,
    version: *const c_char,
}

type RegisterFn = unsafe extern "C" fn() -> RawPluginMetadata;

static LIBS: OnceLock<Mutex<Vec<Library>>> = OnceLock::new();
static REGISTRY: OnceLock<Mutex<Vec<PluginMetadata>>> = OnceLock::new();

fn registry_lock() -> &'static Mutex<Vec<PluginMetadata>> {
    REGISTRY.get_or_init(|| Mutex::new(Vec::new()))
}

fn libs_lock() -> &'static Mutex<Vec<Library>> {
    LIBS.get_or_init(|| Mutex::new(Vec::new()))
}

pub fn registry() -> Vec<PluginMetadata> {
    registry_lock().lock().unwrap().clone()
}

fn cstr_to_string(ptr: *const c_char) -> String {
    if ptr.is_null() {
        String::new()
    } else {
        unsafe { CStr::from_ptr(ptr).to_string_lossy().into_owned() }
    }
}

fn load_library(path: &Path) {
    unsafe {
        match Library::new(path) {
            Ok(lib) => {
                let register: Result<Symbol<RegisterFn>, _> = lib.get(b"register");
                match register {
                    Ok(func) => {
                        let raw = func();
                        let meta = PluginMetadata {
                            name: cstr_to_string(raw.name),
                            version: cstr_to_string(raw.version),
                            path: path.to_path_buf(),
                        };
                        registry_lock().lock().unwrap().push(meta);
                        libs_lock().lock().unwrap().push(lib);
                        tracing::info!("[PLUGIN] loaded {}", path.display());
                    }
                    Err(err) => {
                        tracing::error!(
                            "[PLUGIN] register symbol missing in {}: {}",
                            path.display(),
                            err
                        );
                    }
                }
            }
            Err(err) => {
                tracing::error!("[PLUGIN] failed to load {}: {}", path.display(), err);
            }
        }
    }
}

pub fn load_plugins() {
    let dir = Path::new("./plugins");
    if let Ok(entries) = std::fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if matches!(path.extension().and_then(|e| e.to_str()), Some("so") | Some("dylib")) {
                load_library(&path);
            }
        }
    }
}
