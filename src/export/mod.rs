use crate::state::ExportSummary;
use std::fs::File;
use std::io::Write;

pub fn write_manifest(summary: &ExportSummary) -> std::io::Result<()> {
    let mut file = File::create("exports/manifest.json")?;
    let content = serde_json::to_string_pretty(summary)?;
    file.write_all(content.as_bytes())?;
    Ok(())
}