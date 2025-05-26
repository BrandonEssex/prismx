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
