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
            node_tree: NodeTree::with_mock_data(),
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
                Action::MoveUp => self.move_selection(false),
                Action::MoveDown => self.move_selection(true),
                Action::InputChar(c) => self.node_tree.insert_char(c),
                Action::InputBackspace => self.node_tree.backspace_char(),
                Action::StopEditing => self.node_tree.stop_editing(),
            }
        }

        Ok(())
    }

    pub fn move_selection(&mut self, down: bool) {
        let len = self.node_tree.len();
        if len == 0 {
            return;
        }

        if down {
            self.node_tree.selected_index = (self.node_tree.selected_index + 1).min(len - 1);
        } else {
            self.node_tree.selected_index = self.node_tree.selected_index.saturating_sub(1);
        }
    }
}
