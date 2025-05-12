use chrono::{Datelike, Timelike, Utc};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct CronEntry {
    pub plugin_id: String,
    pub schedule: String, // "every 10m", "at boot", etc.
}

pub struct CronScheduler {
    pub entries: HashMap<String, CronEntry>,
}

impl CronScheduler {
    pub fn new() -> Self {
        Self {
            entries: HashMap::new(),
        }
    }

    pub fn add(&mut self, plugin_id: &str, schedule: &str) {
        self.entries.insert(
            plugin_id.to_string(),
            CronEntry {
                plugin_id: plugin_id.to_string(),
                schedule: schedule.to_string(),
            },
        );
    }

    pub fn should_trigger(&self, plugin_id: &str) -> bool {
        if let Some(entry) = self.entries.get(plugin_id) {
            if entry.schedule == "at boot" {
                return true;
            }
        }
        false
    }
}
