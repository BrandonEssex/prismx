use std::fs;

pub fn load_snapshot() -> Result<(), Box<dyn std::error::Error>> {
    let data = fs::read_to_string("snapshots/compliance_score.json")?;
    println!("[SNAPSHOT] Loaded snapshot: {} bytes", data.len());
    Ok(())
}
