use std::fs;

pub fn load_snapshot() -> Result<(), Box<dyn std::error::Error>> {
    let data = fs::read_to_string("snapshots/compliance_score.json")?;
    println!("[SNAPSHOT] Compliance snapshot OK ({} bytes)", data.len());
    Ok(())
}
