pub mod dashboard;
pub mod dashboard_widgets;
pub mod mindmap;
pub mod mindtrace;
pub mod plugin;
pub mod sandbox;
pub mod export_engine;
pub mod ui;
pub mod keymap;
pub mod icon;
pub mod spotlight;
pub mod clipboard;

use ui::run_ui;

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    run_ui()
}
