use std::sync::{Mutex, OnceLock};

static PLUGIN_HOTKEYS: OnceLock<Mutex<Vec<(String, String)>>> = OnceLock::new();

/// Register a plugin hotkey with a label used by the shortcut overlay.
pub fn register_hotkey(key: impl Into<String>, label: impl Into<String>) {
    let lock = PLUGIN_HOTKEYS.get_or_init(|| Mutex::new(Vec::new()));
    lock.lock().unwrap().push((key.into(), label.into()));
}

/// Retrieve all registered plugin hotkeys.
pub fn get_hotkeys() -> Vec<(String, String)> {
    if let Some(lock) = PLUGIN_HOTKEYS.get() {
        lock.lock().unwrap().clone()
    } else {
        Vec::new()
    }
}
