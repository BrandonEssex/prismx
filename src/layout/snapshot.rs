use crate::state::core::AppState;
use crate::state::serialize::{capture, apply, PersistedLayout};

pub fn save_layout(state: &AppState) -> PersistedLayout {
    capture(state)
}

pub fn load_layout(state: &mut AppState, snap: PersistedLayout) {
    apply(state, snap);
}
