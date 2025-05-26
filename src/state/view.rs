#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ZenViewMode {
    Journal,
    Classic,
    Split,
    Summary,
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

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum OverlayState {
    Hidden,
    Visible,
}

impl Default for OverlayState {
    fn default() -> Self { Self::Hidden }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum PluginViewMode {
    Registry,
}

impl Default for PluginViewMode {
    fn default() -> Self { Self::Registry }
}
