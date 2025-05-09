use crate::plugin::SandboxedPlugin;
use std::collections::VecDeque;
use log::info;

#[derive(Debug, Clone)]
pub struct ReplayEvent {
    pub signal: String,
    pub timestamp: String,
    pub plugin_name: String,
}

pub struct ReplayEngine {
    history: VecDeque<ReplayEvent>,
}

impl ReplayEngine {
    pub fn new() -> Self {
        Self {
            history: VecDeque::new(),
        }
    }

    pub fn log_event(&mut self, plugin: &SandboxedPlugin, signal: &str) {
        let event = ReplayEvent {
            signal: signal.into(),
            timestamp: chrono::Utc::now().to_rfc3339(),
            plugin_name: plugin.name.clone(),
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