// src/action.rs

#[derive(Debug, Clone)]
pub enum Action {
    Quit,
    Redraw,
    ToggleHelp,
    ToggleSidebar,
    ToggleZenMode,
    ToggleLogView,
    ToggleDashboard,
    ToggleMindmap,
    OpenExport,
    Custom(String),
}