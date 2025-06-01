pub mod loader;
pub mod registry;
pub mod state;

pub mod countdown;
pub mod pomodoro;
pub mod api;
pub mod settings;

use ratatui::{backend::CrosstermBackend, layout::Rect, Frame};
use std::io::Stdout;

pub type PluginFrame<'a> = Frame<'a, CrosstermBackend<Stdout>>;

pub trait PluginRender {
    fn render(&mut self, f: &mut PluginFrame<'_>, area: Rect);
}

pub use state::PluginHost;

pub use countdown::CountdownPlugin;
pub use pomodoro::PomodoroPlugin;
