#[derive(Clone, Debug)]
pub enum Action {
    Quit,
    CreateSiblingNode,
    CreateChildNode,
    ToggleZenMode,
    EnterEditNode,
    ExpandNode,
    CollapseNode,
    DuplicateNode,
    DeleteNode,
    ToggleSidebar,
    ToggleCommentPanel,
    ListBookmarks,
    SummarizeNode,
    SuggestLinks,
}
