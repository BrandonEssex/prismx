use std::fs::File;
use std::io::Write;

use serde::Serialize;
use crate::state::ExportSummary;

pub fn export_summary(summary: &ExportSummary, path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let content = serde_json::to_string_pretty(summary)?;
    let mut file = File::create(path)?;
    file.write_all(content.as_bytes())?;
    Ok(())
}