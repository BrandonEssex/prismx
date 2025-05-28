use std::path::Path;

#[derive(Clone)]
pub enum JournalEntry {
    Text(String),
    Image(String),
}

impl JournalEntry {
    pub fn from_input(input: &str) -> Option<Self> {
        let trimmed = input.trim().trim_matches('"');
        let path = trimmed.strip_prefix("file://").unwrap_or(trimmed);
        let ext = Path::new(path)
            .extension()
            .and_then(|e| e.to_str())
            .map(|s| s.to_ascii_lowercase());
        match ext.as_deref() {
            Some("png") | Some("jpg") | Some("jpeg") => {
                Some(JournalEntry::Image(path.to_string()))
            }
            _ => None,
        }
    }

    pub fn display(&self) -> String {
        match self {
            JournalEntry::Text(t) => t.clone(),
            JournalEntry::Image(path) => {
                let name = Path::new(path)
                    .file_name()
                    .unwrap_or_default()
                    .to_string_lossy();
                format!("\u{1F5BC} {}", name)
            }
        }
    }

    pub fn raw_text(&self) -> String {
        match self {
            JournalEntry::Text(t) => t.clone(),
            JournalEntry::Image(p) => p.clone(),
        }
    }
}
