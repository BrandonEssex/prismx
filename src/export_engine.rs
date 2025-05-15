use std::fs::File;
use std::io::Write;
use crate::mindtrace::MindTrace;

pub fn export_to_json(trace: &MindTrace, path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let json = serde_json::to_string_pretty(&trace.nodes)?;
    let mut file = File::create(path)?;
    file.write_all(json.as_bytes())?;
    Ok(())
}
