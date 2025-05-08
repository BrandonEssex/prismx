mod app;
mod config;
mod input;
mod logger;
mod screen;
mod state;
mod zen_mode;
mod json_store;
mod mode;
mod actions;
mod scratchpad;
mod extension_host;
mod spotlight;
mod storage;

fn main() {
    env_logger::init();
    if let Err(e) = app::run() {
        eprintln!("Application error: {:?}", e);
    }
}
