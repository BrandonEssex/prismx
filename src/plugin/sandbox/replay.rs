use std::collections::VecDeque;
use log::info;
use serde::{Serialize, Deserialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct ReplayEvent {
    pub signal: String,
    pub timestamp: String,
    pub plugin_name: String,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct ReplayEngine {
    history: VecDeque<ReplayEvent>,
}

impl ReplayEngine {
    pub fn new() -> Self {
        Self {
            history: VecDeque::new(),
        }
    }

    pub fn log_event(&mut self, plugin_name: &str, signal: &str) {
        let event = ReplayEvent {
            signal: signal.into(),
            timestamp: chrono::Utc::now().to_rfc3339(),
            plugin_name: plugin_name.into(),
        };
        info!("REPLAY: {:?}", event);
        self.history.push_back(event);
        if self.history.len() > 1000 {
            self.history.pop_front();
        }
    }

    pub fn replay_last(&self, count: usize) -> Vec<ReplayEvent> {
        self.history.iter().rev().take(count).cloned().collect()
    }
}