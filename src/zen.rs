use std::fs;
use std::io::Write;
use dirs;
use crate::state::AppState;

impl AppState {
    pub fn export_zen_to_file(&self) {
        let path = dirs::document_dir()
            .unwrap_or_else(|| std::path::PathBuf::from("."))
            .join("prismx")
            .join("zen_export.md");

        let content = self.zen_buffer.join("\n");

        if let Some(parent) = path.parent() {
            let _ = fs::create_dir_all(parent);
        }

        match fs::File::create(&path) {
            Ok(mut file) => {
                let _ = file.write_all(content.as_bytes());
            }
            Err(_) => {
                // Handle the error as needed
            }
        }
    }
}
