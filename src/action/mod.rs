// Author: Brandon Essex
// Action definitions and commands

#[derive(Debug)]
pub enum Action {
    Quit,
    ToggleSidebar,
    SwitchView(crate::state::View),
    PluginCommand(String),
    EditNode,
    CreateNode,
    DeleteNode,
    // Extend with custom user-defined actions as needed
}
