use std::process::Command;

fn main() {
    let target = "wasm32-unknown-unknown";
    let status = Command::new("cargo")
        .args(["build", "--release", "--target", target])
        .status()
        .expect("failed to execute cargo build");
    if !status.success() {
        panic!("wasm build failed");
    }
}
