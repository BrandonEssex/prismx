use serde::{Serialize, Deserialize};
use std::{path::PathBuf, time::SystemTime};
use tracing::{info, debug};

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

impl PermissionSet {
    pub fn grant(&mut self, capability: Capability) {
        debug!("Granting capability: {:?}", capability);
        self.capabilities.push(capability);
    }

    pub fn revoke(&mut self, capability_type: &Capability) {
        debug!("Revoking capability: {:?}", capability_type);
        self.capabilities.retain(|cap| !std::mem::discriminant(cap).eq(&std::mem::discriminant(capability_type)));
    }

    pub fn has_capability(&self, capability: &Capability) -> bool {
        self.capabilities.iter().any(|cap| match (cap, capability) {
            (Capability::Memory(a), Capability::Memory(b)) => a >= b,
            (Capability::Cpu(a), Capability::Cpu(b)) => a >= b,
            (Capability::Filesystem(allowed), Capability::Filesystem(requested)) => {
                requested.iter().all(|p| allowed.contains(p))
            },
            (Capability::Network(a), Capability::Network(b)) => !*b || *a,
            _ => false,
        })
    }

    pub fn enforce(&self, capability: &Capability) -> bool {
        let allowed = self.has_capability(capability);
        if !allowed {
            info!("Capability enforcement triggered, denied: {:?}", capability);
        }
        allowed
    }
}