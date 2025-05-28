// src/render/mod.rs
pub mod zen;
pub mod status;
pub mod shortcuts_overlay;
pub mod spotlight;
pub mod triage;
pub mod module_switcher;
pub mod module_icon;
pub mod favorites;
pub mod zoom_overlay;
pub mod traits;

pub use crate::zen::render::render_zen;
pub use status::render_status_bar;
pub use shortcuts_overlay::render_shortcuts_overlay;
pub use spotlight::render_spotlight;
pub use triage::render_triage;
pub use module_switcher::render_module_switcher;
pub use module_icon::render_module_icon;
pub use favorites::render_favorites_dock;
pub use zoom_overlay::render_zoom_overlay;
pub use traits::Renderable;
pub use crate::settings::render::render_settings;
pub use crate::settings::{SETTING_TOGGLES, SettingToggle, settings_len};

