#[derive(Debug, Clone)]
pub enum Action {
    Quit,
    ToggleZenMode,
    OpenScratchpad,
    ToggleTriage,
    ToggleShortcuts,
    Tick,

    // Mindmap actions
    EnterEditNode,
    OpenContextMenu,
    ToggleMindmapLayout,
    PushEditChar(char),
    PopEditChar,
    CancelEdit,
    CommitEdit,
    NavigateNext,
    NavigatePrev,
}