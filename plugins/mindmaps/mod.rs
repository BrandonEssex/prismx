pub mod nodes;
pub mod state;

pub fn init() {
    println!("[MINDMAPS] Initializing...");
    state::load_mindmap_state();
}
