// FINAL FULL FILE DELIVERY
// Filename: /src/main.rs

mod app;
mod config;
mod screen;
mod input;
mod state;
mod actions;
mod logger;
mod export;
mod spotlight;
mod plugin;
mod tag;
mod ui;
mod zen_mode;
mod dashboard;
mod view_mindmap;
mod view_triage;
mod mindmap_state;
mod log_viewer;
mod shortcut_overlay;
mod storage;
mod dashboard_widgets;

fn main() {
    logger::init_logger();
    if let Err(e) = app::run() {
        eprintln!("Error: {}", e);
    }
}