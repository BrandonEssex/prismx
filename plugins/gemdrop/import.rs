use std::fs;

pub fn scan_incoming() {
    let files = fs::read_dir("incoming/").unwrap_or_else(|_| panic!("Missing incoming/ folder"));
    for file in files.flatten() {
        println!("[GEMDROP] Found: {}", file.path().display());
    }
}
