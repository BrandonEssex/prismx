use std::fs::{self, File};
use std::io::Write;

use dirs;

impl super::AppState {
    pub fn export_zen_to_file(&self) {
        let path = dirs::document_dir()
            .unwrap_or_else(|| std::path::PathBuf::from("."))
            .join("prismx")
            .join("zen_export.md");

        // Clone zen buffer to avoid concurrent mutation
        let lines: Vec<String> = self.zen_buffer.clone();
        let content = lines.join("\n");

        if let Some(parent) = path.parent() {
            let _ = fs::create_dir_all(parent);
        }

        if let Ok(mut file) = File::create(&path) {
            let _ = file.write_all(content.as_bytes());
        }
    }
}
