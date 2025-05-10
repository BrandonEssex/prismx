mod config;
mod state;
mod app;
mod input;
mod screen;
mod actions;
mod export;
mod tag;
mod plugin;
mod zen_mode;
mod dashboard;
mod dashboard_widgets;
mod shortcut_overlay;
mod view_mindmap;
mod view_triage;
mod logger;
mod storage;
mod scratchpad;
mod ui;
mod error_handling;

fn main() {
    let config = config::load_config();
    logger::init_logger(&config);

    if let Err(e) = app::run() {
        eprintln!("Application error: {:?}", e);
    }
}