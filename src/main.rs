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
mod shortcut_overlay;

fn main() {
    if let Err(e) = app::run() {
        eprintln!("Application error: {:?}", e);
    }
}