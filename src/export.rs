use crate::state::{AppState, View};
use std::fs::File;
use std::io::{self, Write};

pub fn export_state(app_state: &AppState) -> io::Result<()> {
    let mut file = File::create("export_state.txt")?;

    writeln!(file, "# PrismX Export")?;
    writeln!(file, "\n## Current View: {:?}", app_state.current_view)?;
    writeln!(file, "Sidebar Visible: {}", app_state.sidebar_view.visible)?;
    writeln!(file, "Node Count: {}\n", app_state.node_tree.len())?;

    for node in &app_state.node_tree.nodes {
        writeln!(
            file,
            "- [{}] {}{}",
            node.id,
            node.label,
            if node.editing { " (editing)" } else { "" }
        )?;
    }

    Ok(())
}
