

pub mod core;
mod navigation;
mod edit;
mod spotlight;
mod history;
mod drag;
mod helpers;

pub mod init;
mod triage;
pub mod view;

pub use core::*;
pub mod serialize;

pub use helpers::register_plugin_favorite;
pub use triage::*;
pub use view::*;

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
