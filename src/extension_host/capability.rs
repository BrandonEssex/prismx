#[derive(Debug, Clone)]
pub enum Capability {
    ReadOnly,
    WriteAccess,
    NetworkAccess,
}
