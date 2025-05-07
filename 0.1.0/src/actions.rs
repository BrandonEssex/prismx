#[derive(Debug, Clone)]
pub enum Action {
    Quit,
    ToggleDebug,
    ToggleZenMode,
    NewInboxEntry,
    MoveUp,
    MoveDown,
    Enter,
    Exit,
    Backspace,
    Char(char),
    Ctrl(char),
}