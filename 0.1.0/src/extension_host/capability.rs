use serde::{Serialize, Deserialize};
use std::{path::PathBuf, time::SystemTime};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Capability {
    Memory(usize),
    Cpu(u64),
    Filesystem(Vec<PathBuf>),
    Network(bool),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PermissionSet {
    pub capabilities: Vec<Capability>,
    pub granted_at: SystemTime,
    pub expires_at: Option<SystemTime>,
}

impl Default for PermissionSet {
    fn default() -> Self {
        Self {
            capabilities: vec![
                Capability::Memory(16 * 1024 * 1024),
                Capability::Cpu(50_000_000),
                Capability::Filesystem(vec![]),
                Capability::Network(false),
            ],
            granted_at: SystemTime::now(),
            expires_at: None,
        }
    }
}