use crate::extension_host::errors::{ExtensionHostError, Result};
use serde::Deserialize;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Deserialize)]
pub struct PluginManifest {
    pub name: String,
    pub author: String,
    pub version: String,
    pub prismx_api_version: String,
    pub entrypoint: String,
}

#[derive(Debug)]
pub struct Plugin {
    pub manifest: PluginManifest,
    pub wasm_path: PathBuf,
}

impl Plugin {
    pub fn from_path<P: AsRef<Path>>(path: P) -> Result<Self> {
        let path = path.as_ref();
        let manifest_path = path.join("prismx-plugin.json");

        let manifest_str = fs::read_to_string(&manifest_path)
            .map_err(|e| ExtensionHostError::ManifestNotFound(e.to_string()))?;
        let manifest: PluginManifest =
            serde_json::from_str(&manifest_str).map_err(|e| ExtensionHostError::ManifestParseError(e.to_string()))?;

        let wasm_path = path.join(&manifest.entrypoint);
        if !wasm_path.exists() {
            return Err(ExtensionHostError::WasmBinaryNotFound(
                wasm_path.display().to_string(),
            ));
        }

        Ok(Self { manifest, wasm_path })
    }
}