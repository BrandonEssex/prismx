// extension.rs
#[derive(Debug)]
pub struct ExtensionMeta {
    pub name: String,
    pub version: String,
    pub author: String,
    pub capabilities: Vec<String>,
}
