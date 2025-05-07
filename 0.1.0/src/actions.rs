use crate::inbox::Priority;

#[derive(Debug, Clone)]
pub enum Action {
    // Global controls
    Quit,
    Help,
    ToggleDebug,
    Redraw,

    // Navigation
    Up,
    Down,
    Left,
    Right,
    Enter,
    Back,

    // Mode toggles
    ToggleTriage,
    ToggleZenMode,
    ToggleSpotlight,
    ToggleSettings,

    // Triage operations
    NewInboxEntry,
    AssignInboxTask(String),
    SetTaskPriority(String, Priority),
    TagInboxTask(String, Vec<String>),
    ArchiveTask(String),
    TriageTask(String),

    // Runtime-specific input variants
    Ctrl(char),
    Input(char),
    Submit,
    Backspace,
    MoveUp,
    MoveDown,
    Exit,
    OpenScratchpad,
    Char(char),
}