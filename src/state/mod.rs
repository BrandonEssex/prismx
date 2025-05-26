pub mod hotkeys;
pub use hotkeys::*;

mod core;
mod navigation;
mod edit;
mod zen;
mod spotlight;
mod history;
mod drag;
mod helpers;

pub use core::*;

pub use helpers::register_plugin_favorite;
