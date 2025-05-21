use std::fs;
use std::io::Write;

use dirs;

impl super::AppState {
    pub fn export_zen_to_file(&self) {
        let path = dirs::document_dir()
            .unwrap_or_else(|| std::path::PathBuf::from("."))
            .join("prismx")
            .join("zen_export.md");

        let content = self.zen_buffer.join("\n");

        if let Some(parent) = path.parent() {
            let _ = fs::create_dir_all(parent);
        }

        if let Ok(mut file) = fs::File::create(&path) {
            let _ = file.write_all(content.as_bytes());
        }
    }
}
