pub mod dashboard;
pub mod dashboard_widgets;
pub mod mindmap;
pub mod mindtrace;
pub mod plugin;
pub mod sandbox;
pub mod export_engine;
pub mod ui;
pub mod keymap;
pub mod dashboard_toggle;
pub mod icon;

use ui::run_ui;

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    run_ui()
}
