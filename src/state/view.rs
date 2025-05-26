#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ZenViewMode {
    Journal,
    Classic,
    Split,
}

impl Default for ZenViewMode {
    fn default() -> Self { Self::Journal }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum TriageViewMode {
    Feed,
    Actions,
}

impl Default for TriageViewMode {
    fn default() -> Self { Self::Feed }
}
