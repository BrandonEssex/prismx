// CLEANED: src/main.rs

fn main() {
    if let Err(e) = prismx::app::run() {
        eprintln!("Error: {}", e);
    }
}