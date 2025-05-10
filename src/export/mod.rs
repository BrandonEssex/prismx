use crate::state::ExportSummary;
use std::fs::File;
use std::io::Write;

pub fn write_export_summary(summary: &ExportSummary, path: &str) -> std::io::Result<()> {
    let content = serde_json::to_string_pretty(summary)?;
    let mut file = File::create(path)?;
    file.write_all(content.as_bytes())?;
    Ok(())
}