use crate::inbox::Priority;

#[derive(Debug, Clone)]
pub enum Action {
    Quit,
    Help,
    ToggleDebug,
    Redraw,
    Up,
    Down,
    Left,
    Right,
    Enter,
    Back,

    ToggleTriage,
    ToggleZenMode,
    ToggleSpotlight,
    ToggleSettings,

    NewInboxEntry,
    AssignInboxTask(String),
    SetTaskPriority(String, Priority),
    TagInboxTask(String, Vec<String>),
    ArchiveTask(String),
    TriageTask(String),

    OpenScratchpad,
}