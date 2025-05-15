use prismx::run;

fn main() {
    println!("Launching PrismX...");
    if let Err(e) = run() {
        eprintln!("Error running app: {:?}", e);
    }
}
