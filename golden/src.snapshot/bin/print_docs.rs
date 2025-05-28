fn main() {
    match dirs::document_dir() {
        Some(path) => tracing::info!("🗂 macOS resolved document path: {:?}", path),
        None => tracing::info!("❌ Could not resolve Documents directory"),
    }
}
