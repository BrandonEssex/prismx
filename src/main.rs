mod app;
mod state;
mod input;
mod config;
mod screen;
mod actions;
mod spotlight;
mod extension_host;
mod mindmap_state;
mod view_mindmap;
mod view_triage;
mod storage;
mod scratchpad;
mod zen_mode;
mod dashboard;
mod dashboard_widgets;
mod shortcut_overlay;
mod logger;
mod log_viewer;
mod unlock;
mod routine_forge;
mod template_engine;

fn main() {
    logger::init_logger();

    if let Err(e) = app::run() {
        eprintln!("Application error: {:?}", e);
        log::error!("Fatal error: {:?}", e);
    }
}