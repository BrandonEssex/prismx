#[derive(Debug, Clone)]
pub enum Capability {
    FileRead,
    FileWrite,
    NetworkAccess,
    RenderOverlay,
    RegisterKeybind,
    EmitSignal,
}