pub mod loader;
pub mod interface;
pub mod state;

pub mod countdown;
pub mod pomodoro;

pub use interface::{PluginFrame, PluginRender};
pub use loader::{load_plugin, load_plugins, discover_plugins, LoadedPlugin};
pub use state::PluginHost;

pub use countdown::CountdownPlugin;
pub use pomodoro::PomodoroPlugin;
