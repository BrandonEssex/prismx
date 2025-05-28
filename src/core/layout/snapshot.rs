#[cfg(feature = "std")]
use crate::state::core::AppState;
#[cfg(feature = "std")]
use crate::state::serialize::{capture, apply, PersistedLayout};

#[cfg(feature = "std")]
pub fn save_layout(state: &AppState) -> PersistedLayout {
    capture(state)
}

#[cfg(feature = "std")]
pub fn load_layout(state: &mut AppState, snap: PersistedLayout) {
    apply(state, snap);
}
