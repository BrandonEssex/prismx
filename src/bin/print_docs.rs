fn main() {
    match dirs::document_dir() {
        Some(path) => println!("🗂 macOS resolved document path: {:?}", path),
        None => println!("❌ Could not resolve Documents directory"),
    }
}
