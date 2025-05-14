#[derive(Debug)]
pub enum Action {
    Quit,
    ToggleSidebar,
    SwitchView(crate::state::View),
    PluginCommand(String),
    EditNode,
    CreateNode,
    DeleteNode,
    MoveUp,
    MoveDown,
    InputChar(char),
    InputBackspace,
    StopEditing,
}
