use serde::Deserialize;
use std::fs;
use std::error::Error;
use std::sync::{Mutex, OnceLock};
use std::process::Command;
use std::time::{Duration, Instant};
use crate::state::PluginTagFilter;

#[derive(Debug, Deserialize, Clone)]
pub struct PluginManifest {
    pub id: String,
    pub name: String,
    pub description: String,
    pub url: String,
    pub tags: Vec<String>,
    #[serde(default)]
    pub trusted: bool,
    #[serde(default)]
    pub trust_chain: Option<String>,
    pub version: String,                      
}



#[derive(Debug, Deserialize)]
struct PluginRegistry {
    #[serde(rename = "plugin")]
    pub plugins: Vec<PluginManifest>,
}

pub fn load_registry() -> Result<Vec<PluginManifest>, Box<dyn Error>> {
    let data = fs::read_to_string("config/prismx-epel.toml")?;
    let registry: PluginRegistry = toml::from_str(&data)?;
    Ok(registry.plugins)
}

// ───── Registry Sync Logic ─────

struct RegistryState {
    last_hash: Option<String>,
    last_check: Instant,
    last_sync: Option<Instant>,
}

static REGISTRY_STATE: OnceLock<Mutex<RegistryState>> = OnceLock::new();

fn compute_hash() -> Option<String> {
    let output = Command::new("sha256sum")
        .arg("config/prismx-epel.toml")
        .output()
        .ok()?;
    let hash = String::from_utf8_lossy(&output.stdout);
    hash.split_whitespace().next().map(|s| s.to_string())
}

fn init_state() -> RegistryState {
    RegistryState {
        last_hash: compute_hash(),
        last_check: Instant::now(),
        last_sync: None,
    }
}

pub fn init() {
    REGISTRY_STATE.get_or_init(|| Mutex::new(init_state()));
}

pub fn tick() {
    let lock = REGISTRY_STATE.get_or_init(|| Mutex::new(init_state()));
    let mut state = lock.lock().unwrap();
    if state.last_check.elapsed() < Duration::from_secs(2) {
        return;
    }
    state.last_check = Instant::now();
    let new_hash = match compute_hash() {
        Some(h) => h,
        None => return,
    };
    if state.last_hash.as_deref() != Some(&new_hash) {
        state.last_hash = Some(new_hash);
        if load_registry().is_ok() {
            tracing::info!("[PLUGIN] registry synchronized");
        }
        state.last_sync = Some(Instant::now());
    }
}

pub fn sync_badge() -> bool {
    if let Some(lock) = REGISTRY_STATE.get() {
        let state = lock.lock().unwrap();
        if let Some(t) = state.last_sync {
            return t.elapsed() < Duration::from_secs(3);
        }
    }
    false
}

// ───── Filtering ─────

pub fn registry_filtered(filter: PluginTagFilter) -> Vec<PluginManifest> {
    match load_registry() {
        Ok(registry) => registry
            .into_iter()
            .filter(|p| match filter {
                PluginTagFilter::All => true,
                PluginTagFilter::Trusted => p.trusted,
                PluginTagFilter::Debug => p.tags.iter().any(|t| t == "debug"),
            })
            .collect(),
        Err(_) => vec![],
    }
}
