mod actions;
mod app;
mod config;
mod dashboard;
mod dashboard_widgets;
mod error_handling;
mod extension_host;
mod export;
mod input;
mod logger;
mod mindmap_state;
mod plugin;
mod screen;
mod scratchpad;
mod shortcut_overlay;
mod spotlight;
mod state;
mod storage;
mod tag;
mod ui;
mod view_mindmap;
mod view_triage;
mod zen_mode;

fn main() {
    if let Err(e) = app::run() {
        eprintln!("Application error: {:?}", e);
    }
}