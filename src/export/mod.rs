// src/export/mod.rs

use std::fs::File;
use std::io::{self, Write};
use std::path::Path;

use crate::state::AppState;

#[derive(Debug)]
pub struct ExportSummary {
    pub total_nodes: usize,
    pub total_plugins: usize,
    pub notes: String,
}

pub fn write_export_summary<P: AsRef<Path>>(
    path: P,
    summary: &ExportSummary,
    app_state: &AppState,
) -> io::Result<()> {
    let mut file = File::create(path)?;
    writeln!(file, "# Export Summary")?;
    writeln!(file, "- Total Nodes: {}", summary.total_nodes)?;
    writeln!(file, "- Total Plugins: {}", summary.total_plugins)?;
    writeln!(file, "- Notes: {}", summary.notes)?;
    writeln!(file, "\n## Active View: {:?}", app_state.view)?;
    writeln!(file, "Sidebar: {:?}", app_state.sidebar)?;
    if let Some(plugin) = &app_state.active_plugin {
        writeln!(file, "Active Plugin: {}", plugin)?;
    }
    writeln!(file, "Focused Node: {:?}", app_state.focused_node)?;
    Ok(())
}