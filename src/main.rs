mod app;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    app::launch()?;
    Ok(())
}