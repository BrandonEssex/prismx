use std::fs;

pub fn check_retirement() -> Result<(), Box<dyn std::error::Error>> {
    let retire = fs::read_to_string("policy/retire.toml")?;
    tracing::info!("[RETIRE] Lockout policy loaded: {} bytes", retire.len());
    Ok(())
}
