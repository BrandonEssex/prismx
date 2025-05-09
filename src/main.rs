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
mod storage;
mod scratchpad;

fn main() {
    if let Err(e) = app::run() {
        eprintln!("Application error: {:?}", e);
    }
}