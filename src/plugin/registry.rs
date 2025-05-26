#[derive(Clone, Copy)]
pub struct PluginEntry {
    pub name: &'static str,
    pub version: &'static str,
    pub description: &'static str,
    pub trusted: bool,
    pub trust_chain: &'static str,
    pub tags: &'static [&'static str],
}

use std::sync::OnceLock;
use std::process::Command;
use std::sync::Mutex;
use std::time::{Duration, Instant};

struct RegistryState {
    last_hash: Option<String>,
    last_check: Instant,
    last_sync: Option<Instant>,
}

static REGISTRY_STATE: OnceLock<Mutex<RegistryState>> = OnceLock::new();

fn compute_hash() -> Option<String> {
    let output = Command::new("sha256sum")
        .arg("config/plugin.json")
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

/// Initialize plugin registry sync state.
pub fn init() {
    REGISTRY_STATE.get_or_init(|| Mutex::new(init_state()));
}

/// Check registry file for updates. Should be called periodically.
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
        if crate::config::load_locked_registry().is_ok() {
            tracing::info!("[PLUGIN] registry synchronized");
        }
        state.last_sync = Some(Instant::now());
    }
}

/// Returns true if a sync occurred recently.
pub fn sync_badge() -> bool {
    if let Some(lock) = REGISTRY_STATE.get() {
        let state = lock.lock().unwrap();
        if let Some(t) = state.last_sync {
            return t.elapsed() < Duration::from_secs(3);
        }
    }
    false
}

pub fn registry() -> Vec<PluginEntry> {
    vec![
        PluginEntry {
            name: "GemX",
            version: "0.1.0",
            description: "Mindmap engine",
            trusted: true,
            trust_chain: "PrismX Core",
            tags: &["editor", "trusted"],
        },
        PluginEntry {
            name: "Dashboard",
            version: "0.1.0",
            description: "Project dashboard",
            trusted: true,
            trust_chain: "PrismX Core",
            tags: &["utility", "trusted"],
        },
        PluginEntry {
            name: "Mindtrace",
            version: "0.1.0",
            description: "AI memory system",
            trusted: false,
            trust_chain: "unknown",
            tags: &["debug"],
        },
        PluginEntry {
            name: "RoutineForge",
            version: "0.1.0",
            description: "Task & habit manager",
            trusted: false,
            trust_chain: "unknown",
            tags: &["utility"],
        },
    ]
}

use crate::state::PluginTagFilter;

pub fn registry_filtered(filter: PluginTagFilter) -> Vec<PluginEntry> {
    registry()
        .into_iter()
        .filter(|p| match filter {
            PluginTagFilter::All => true,
            PluginTagFilter::Trusted => p.trusted,
            PluginTagFilter::Debug => p.tags.iter().any(|t| *t == "debug"),
        })
        .collect()
}
