pub mod input;
pub mod control;
pub mod ui;

pub use input::handle_keys as handle_module_switcher_keys;
pub use control::{open_switcher, close_switcher, next_module, select_current};
pub use ui::render_module_switcher;
