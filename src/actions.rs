// FINAL FULL FILE DELIVERY
// Filename: /src/actions.rs

#[derive(Debug, Clone)]
pub enum Action {
    Quit,
    ToggleZenMode,
    ToggleShortcuts,
    ToggleLogViewer,
    ToggleTriage,
    EnterEditNode,
    PushEditChar(char),
    PopEditChar,
    CommitEdit,
    CancelEdit,
    NavigateNext,
    NavigatePrev,
    CreateSiblingNode,
    CreateChildNode,
    DeleteNode,
    DuplicateNode,
    ToggleTimelineView,
    ToggleMarkdownPreview,
    ToggleTagFilterMenu,
    SearchNode,
    ExpandNode,
    CollapseNode,
    SwitchWorkspace,
}
