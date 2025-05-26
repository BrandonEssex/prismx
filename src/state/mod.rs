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

impl AppState {
    /// Handle an Enter keypress in mindmap mode by creating a sibling node.
    pub fn handle_enter_key(&mut self) {
        self.add_sibling_node();
    }

    /// Handle a Tab keypress in mindmap mode by creating a child node.
    pub fn handle_tab_key(&mut self) {
        self.add_child_node();
    }
}
