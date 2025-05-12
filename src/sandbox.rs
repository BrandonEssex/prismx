// src/sandbox.rs

use std::collections::HashSet;

#[derive(Debug, Default)]
pub struct CapabilitySandbox {
    granted: HashSet<String>,
}

impl CapabilitySandbox {
    pub fn new() -> Self {
        Self {
            granted: HashSet::new(),
        }
    }

    pub fn allow(&mut self, capability: &str) {
        self.granted.insert(capability.to_string());
    }

    pub fn revoke(&mut self, capability: &str) {
        self.granted.remove(capability);
    }

    pub fn is_allowed(&self, capability: &str) -> bool {
        self.granted.contains(capability)
    }

    pub fn all(&self) -> Vec<String> {
        self.granted.iter().cloned().collect()
    }
}