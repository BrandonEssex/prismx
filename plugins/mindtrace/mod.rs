pub mod memory;

pub fn init() {
    println!("[MINDTRACE] Memory engine loaded.");
    memory::load_memory();
}
