use std::fs::File;
use std::io::Write;

use crate::state::ExportSummary;

pub fn write_manifest(summary: &ExportSummary) -> std::io::Result<()> {
    let content = serde_json::to_string_pretty(summary)?;
    let mut file = File::create("export_manifest.json")?;
    file.write_all(content.as_bytes())?;
    Ok(())
}