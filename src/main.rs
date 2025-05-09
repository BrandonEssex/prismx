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

fn main() {
    logger::init_logger();
    if let Err(e) = app::run() {
        eprintln!("Error: {}", e);
    }
}