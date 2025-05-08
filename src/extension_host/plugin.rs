use std::fs;
use std::path::{Path, PathBuf};

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct PluginManifest {
    pub name: String,
    pub author: String,
    pub version: String,
    pub prismx_api_version: String,
    pub entrypoint: String,
}

pub struct Plugin {
    pub manifest: PluginManifest,
    pub wasm_bytes: Vec<u8>,
}

impl Plugin {
    pub fn from_path<P: AsRef<Path>>(path: P) -> Result<Self, String> {
        let manifest_path = path.as_ref().join("prismx-plugin.json");
        let wasm_path = path.as_ref().join("plugin.wasm");

        let manifest_str = fs::read_to_string(&manifest_path)
            .map_err(|e| format!("Failed to read manifest: {}", e))?;
        let manifest: PluginManifest = serde_json::from_str(&manifest_str)
            .map_err(|e| format!("Failed to parse manifest: {}", e))?;

        let wasm_bytes = fs::read(&wasm_path)
            .map_err(|e| format!("Failed to read WASM binary: {}", e))?;

        Ok(Self { manifest, wasm_bytes })
    }
}