mod mode;
mod app;
mod config;
mod error_handling;
mod input;
mod logger;
mod screen;
mod state;

mod zen_mode;
mod scratchpad;
mod inbox;
mod view_triage;

mod spotlight;
mod tui;
mod api;
mod storage;
mod extension_host;
mod actions;
mod util;

use crate::app::run_app;

fn main() -> anyhow::Result<()> {
    logger::init_logging();
    run_app()
}