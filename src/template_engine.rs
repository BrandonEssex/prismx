use std::fs;
use std::path::Path;
use serde_json::Value;
use crate::storage::inbox_storage::{InboxState, TriageItem};
use chrono::Utc;

pub fn apply_template_to_inbox(inbox: &mut InboxState, path: &Path) -> std::io::Result<()> {
    let content = fs::read_to_string(path)?;
    let value: Value = serde_json::from_str(&content)?;

    if let Some(array) = value.get("tasks").and_then(|v| v.as_array()) {
        for entry in array {
            if let Some(title) = entry.get("title").and_then(|v| v.as_str()) {
                let item = TriageItem {
                    id: uuid::Uuid::new_v4().to_string(),
                    title: title.into(),
                    shard: "auto".into(),
                    tags: vec![],
                    priority: 2,
                    created: Utc::now(),
                };
                inbox.items.push(item);
            }
        }
    }

    Ok(())
}