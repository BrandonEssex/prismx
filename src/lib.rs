pub mod action;
pub mod app;
pub mod command_bar;
pub mod config;
pub mod dashboard;
pub mod dashboard_widgets;
pub mod error_handling;
pub mod export;  // Only export/mod.rs should exist
pub mod extension_host;
pub mod input;
pub mod inbox;
pub mod json_store;
pub mod logger;
pub mod log_viewer;
pub mod mindmap_state;
pub mod mode;
pub mod plugin;
pub mod routine_forge;
pub mod sandbox; // Must exist in src/sandbox.rs or sandbox/mod.rs
pub mod scratchpad;
pub mod screen;
pub mod shortcut_overlay;
pub mod spotlight;
pub mod state;
pub mod status_bar; // Must exist in src/status_bar.rs or mod.rs
pub mod storage;
pub mod tag;
pub mod template_engine;
pub mod timer;
pub mod tui;
pub mod ui;
pub mod unlock;
pub mod util; // Only one of util.rs or util/mod.rs must exist
pub mod view_export;
pub mod view_mindmap;
pub mod view_triage;
pub mod zen_mode;