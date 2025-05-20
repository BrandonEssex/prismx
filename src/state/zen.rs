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

        match fs::File::create(&path) {
            Ok(mut file) => {
                if let Err(err) = file.write_all(content.as_bytes()) {
                    eprintln!("❌ Write failed: {}", err);
                } else {
                    println!("✅ Zen exported to: {:?}", path);
                }
            }
            Err(e) => {
                eprintln!("❌ File create failed: {}", e);
            }
        }
    }
}
