pub mod nodes;
pub mod state;

pub fn init() {
    println!("[GEMX] Mindmap plugin initialized.");
    state::load();
}
