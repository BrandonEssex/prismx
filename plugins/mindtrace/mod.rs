pub mod memory;

pub fn init() {
    println!("[MINDTRACE] AI memory system initialized.");
    memory::load_memory();
}
