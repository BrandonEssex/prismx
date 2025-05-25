use prismx::dynamic_plugin::{PrismPlugin};
use prismx::state::AppState;

pub struct DummyPlugin;

impl PrismPlugin for DummyPlugin {
    fn register(state: &mut AppState) {
        prismx::register_plugin_favorite(state, "💡", "dummy");
    }
}

#[no_mangle]
pub extern "C" fn prismx_register(state: &mut AppState) {
    DummyPlugin::register(state);
}
