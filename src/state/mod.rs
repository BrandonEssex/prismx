

pub mod core;
mod navigation;
mod edit;
mod spotlight;
mod history;
mod drag;
mod helpers;
pub mod links;

pub mod init;
mod triage;
pub mod view;
pub mod config;

pub use core::*;
pub mod serialize;

pub use helpers::register_plugin_favorite;
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

    /// Handle Shift+Tab by promoting the current node one level up.
    pub fn handle_shift_tab_key(&mut self) {
        self.promote_selected_node();
    }

    /// Generate the next automatic node label.
    pub fn next_label(&mut self) -> String {
        let label = format!("node {:03}", self.next_node_label);
        self.next_node_label += 1;
        label
    }
}
