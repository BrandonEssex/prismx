pub mod nodes;
pub mod state;

pub fn init() {
    println!("[GEMX] Mindmap system loaded.");
    state::load();
}
