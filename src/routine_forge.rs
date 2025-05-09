use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Routine {
    pub name: String,
    pub template_path: String,
    pub repeat: String, // "once", "daily", "weekly"
    pub last_run: Option<DateTime<Utc>>,
}

pub struct RoutineForge {
    pub routines: Vec<Routine>,
}

impl RoutineForge {
    pub fn load() -> Self {
        let path = "data/routines.json";
        if let Ok(content) = fs::read_to_string(path) {
            if let Ok(routines) = serde_json::from_str(&content) {
                return Self { routines };
            }
        }
        Self { routines: vec![] }
    }

    pub fn save(&self) {
        if let Ok(json) = serde_json::to_string_pretty(&self.routines) {
            let _ = fs::write("data/routines.json", json);
        }
    }

    pub fn run(&mut self, name: &str) {
        if let Some(r) = self.routines.iter_mut().find(|r| r.name == name) {
            let now = Utc::now();
            log::info!("Running routine: {}", r.name);
            r.last_run = Some(now);
            self.save();
        }
    }
}