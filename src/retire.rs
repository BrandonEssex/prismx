use std::fs;

pub fn check_retirement() -> Result<(), Box<dyn std::error::Error>> {
    let retire = fs::read_to_string("policy/retire.toml")?;
    println!("[RETIRE] Lockout policy loaded: {} bytes", retire.len());
    Ok(())
}
