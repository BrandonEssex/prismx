#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Capability {
    ReadFilesystem,
    WriteFilesystem,
    AccessNetwork,
    AccessClipboard,
    ModifyState,
    ExecuteShell,
    Custom(String),
}

#[derive(Debug, Default)]
pub struct PermissionSet {
    granted: std::collections::HashSet<Capability>,
}

impl PermissionSet {
    pub fn grant(&mut self, capability: Capability) {
        self.granted.insert(capability);
    }

    pub fn revoke(&mut self, capability: &Capability) {
        self.granted.remove(capability);
    }

    pub fn has(&self, capability: &Capability) -> bool {
        self.granted.contains(capability)
    }

    pub fn enforce(&self, capability: &Capability) -> Result<(), String> {
        if self.has(capability) {
            Ok(())
        } else {
            Err(format!("Permission denied: {:?}", capability))
        }
    }
}