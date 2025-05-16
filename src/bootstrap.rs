use crate::{config, plugin, tui, trust, federation, snapshot, retire};

/// Public entrypoint used by `src/bin/prismx.rs`
pub fn start() -> Result<(), Box<dyn std::error::Error>> {
    println!("[BOOTSTRAP] Starting PrismX v10.0.0...");

    // Validate trust system
    trust::verify_trust_chains()?;

    // Load and lock plugin registry
    let registry = config::load_locked_registry()?;
    println!("[BOOTSTRAP] Plugin registry loaded ({} plugins)", registry.plugins.len());

    // Load and verify all plugins
    plugin::load_all_plugins()?;

    // Start federation sync if enabled
    federation::start_sync_if_enabled();

    // Load snapshot state (optional, will fallback)
    snapshot::load_snapshot().ok();

    // Check retirement lockout conditions
    retire::check_retirement().ok();

    // Launch terminal UI
    tui::launch_ui()?;

    Ok(())
}
