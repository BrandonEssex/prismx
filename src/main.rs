#[cfg(feature = "std")]
fn main() -> std::io::Result<()> {
    prismx::bootstrap::start()
}

#[cfg(not(feature = "std"))]
fn main() {}
