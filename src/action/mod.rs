#[derive(Debug)]
pub enum Action {
    Quit,
    ToggleSidebar,
    SwitchView(crate::state::View),
    PluginCommand(String),
    EditNode,
    CreateNode,
    DeleteNode,
}