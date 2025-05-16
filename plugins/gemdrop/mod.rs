pub mod import;
pub mod resolve;

pub fn activate() {
    println!("[GEMDROP] Import activated.");
    import::scan_incoming();
}
