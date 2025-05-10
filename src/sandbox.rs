use crate::state::AppState;

pub fn enter_sandbox_mode(state: &mut AppState) {
    // Simulate entering a limited/safe runtime space
    println!("Entering sandbox mode...");
    state.layout_profile = "sandbox".to_string();
}