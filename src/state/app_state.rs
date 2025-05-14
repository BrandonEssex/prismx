// Author: Brandon Essex
// Core application state for PrismX/GemX

use crate::state::{SidebarView, View};
use crate::node_tree::NodeTree;
use crate::plugin::PluginManager;

pub struct AppState {
    pub should_quit: bool,
    pub current_view: View,
    pub sidebar_view: SidebarView,
    pub node_tree: NodeTree,
    pub plugin_manager: PluginManager,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            should_quit: false,
            current_view: View::Mindmap,
            sidebar_view: SidebarView::default(),
            node_tree: NodeTree::default(),
            plugin_manager: PluginManager::default(),
        }
    }
}

impl AppState {
    pub fn handle_input_event(&mut self) -> Result<(), std::io::Error> {
        use crate::input::map_input_to_action;
        use crate::action::Action;

        if let Some(action) = map_input_to_action()? {
            match action {
                Action::Quit => self.should_quit = true,
                Action::ToggleSidebar => self.sidebar_view.toggle(),
                Action::SwitchView(view) => self.current_view = view,
                Action::PluginCommand(cmd) => self.plugin_manager.handle_command(cmd),
                Action::EditNode => self.node_tree.begin_editing_selected(),
                Action::CreateNode => self.node_tree.create_child_node(),
                Action::DeleteNode => self.node_tree.delete_selected(),
                _ => {}
            }
        }

        Ok(())
    }
}
