use serde::Deserialize;
use super::errors::{ExtensionHostError, Result};
use std::{fs, path::Path};

const PRISMX_API_VERSION: &str = "0.1.0";

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
    pub fn from_path<P: AsRef<Path>>(path: P) -> Result<Self> {
        let manifest_path = path.as_ref().join("prismx-plugin.json");
        let wasm_path = path.as_ref().join("plugin.wasm");

        let manifest_str = fs::read_to_string(&manifest_path)
            .map_err(|_| ExtensionHostError::ManifestNotFound(manifest_path.display().to_string()))?;

        let manifest: PluginManifest = serde_json::from_str(&manifest_str)
            .map_err(|e| ExtensionHostError::ManifestParseError(e.to_string()))?;

        if manifest.prismx_api_version != PRISMX_API_VERSION {
            return Err(ExtensionHostError::IncompatibleApiVersion {
                expected: PRISMX_API_VERSION.into(),
                found: manifest.prismx_api_version,
            });
        }

        let wasm_bytes = fs::read(&wasm_path)
            .map_err(|_| ExtensionHostError::WasmBinaryNotFound(wasm_path.display().to_string()))?;

        Ok(Self { manifest, wasm_bytes })
    }
}