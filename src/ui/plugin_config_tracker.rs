use std::collections::HashMap;

#[derive(Debug)]
pub struct ConfigHistory {
    pub history: HashMap<String, Vec<String>>,
}

impl ConfigHistory {
    pub fn new() -> Self {
        Self {
            history: HashMap::new(),
        }
    }

    pub fn record(&mut self, plugin_id: &str, snapshot: &str) {
        self.history
            .entry(plugin_id.to_string())
            .or_default()
            .push(snapshot.to_string());
    }

    pub fn latest(&self, plugin_id: &str) -> Option<&String> {
        self.history.get(plugin_id)?.last()
    }
}
