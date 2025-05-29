pub mod loader {
    pub use crate::plugins::loader::*;
}

pub use crate::plugins::state::PluginHost;

pub mod panel;
pub mod registry;
pub mod hook;
