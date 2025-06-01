use once_cell::sync::OnceCell;
use std::sync::Mutex;

/// A settings tab provided by a plugin.
pub struct PluginSettingsTab {
    /// Title displayed in the settings tab bar.
    pub title: String,
    /// Callback executed when this tab is active.
    pub render_fn: Box<dyn FnMut() + Send>,
}

static PLUGIN_TABS: OnceCell<Mutex<Vec<PluginSettingsTab>>> = OnceCell::new();

/// Register a plugin-provided settings tab.
pub fn register_tab(title: impl Into<String>, render_fn: Box<dyn FnMut() + Send>) {
    let lock = PLUGIN_TABS.get_or_init(|| Mutex::new(Vec::new()));
    lock.lock().unwrap().push(PluginSettingsTab { title: title.into(), render_fn });
}

/// Consume and return all registered plugin tabs.
/// Intended to be called during application startup.
pub fn drain_tabs() -> Vec<PluginSettingsTab> {
    let lock = PLUGIN_TABS.get_or_init(|| Mutex::new(Vec::new()));
    lock.lock().unwrap().drain(..).collect()
}
