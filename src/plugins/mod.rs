pub mod loader;
pub mod interface;
pub mod state;

pub mod countdown;
pub mod pomodoro;
pub mod api;

pub use interface::{PluginFrame, PluginRender};
pub use state::PluginHost;

pub use countdown::CountdownPlugin;
pub use pomodoro::PomodoroPlugin;
