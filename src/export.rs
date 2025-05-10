// FINAL FULL FILE DELIVERY
// Filename: /src/export.rs
// File Delivery Progress: 13/∞ FINAL FILES delivered

use std::fs::File;
use std::io::Write;
use crate::state::ExportSummary;
use serde::Serialize;

pub fn export_summary(summary: &ExportSummary, path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let content = serde_json::to_string_pretty(summary)?;
    let mut file = File::create(path)?;
    file.write_all(content.as_bytes())?;
    Ok(())
}
