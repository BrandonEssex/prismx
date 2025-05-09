#[derive(Debug, Clone)]
pub enum Action {
    Quit,
    ToggleZenMode,
    OpenScratchpad,
    ToggleTriage,
    ToggleShortcuts,
    ToggleLogViewer,
    Tick,

    // Mindmap core
    EnterEditNode,
    OpenContextMenu,
    ToggleMindmapLayout,
    PushEditChar(char),
    PopEditChar,
    CancelEdit,
    CommitEdit,
    NavigateNext,
    NavigatePrev,

    // Node creation/manipulation
    CreateSiblingNode,
    CreateChildNode,
    DuplicateNode,
    DeleteNode,

    // Navigation
    NavigateParent,
    NavigateChild,
    NavigateLeft,
    NavigateRight,

    // View / Interaction
    ToggleMarkdownPreview,
    ToggleTimelineView,
    ToggleTagFilterMenu,
    CollapseNode,
    ExpandNode,
    TriggerTemplate(String),
    SwitchWorkspace,
    SearchNode,
}