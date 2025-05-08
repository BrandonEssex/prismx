mod app;
mod config;
mod input;
mod logger;
mod screen;
mod state;
mod zen_mode;
mod json_store;
mod mode;
mod extension_host;

fn main() {
    logger::init().expect("Failed to initialize logger");
    if let Err(e) = app::run() {
        eprintln!("Application error: {:?}", e);
    }
}
