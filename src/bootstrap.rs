use crate::{config, plugin, tui, trust, federation, snapshot, retire};

pub fn start() -> Result<(), Box<dyn std::error::Error>> {
    println!("[BOOTSTRAP] PrismX v10.1.0+Final Starting...");

    trust::verify_trust_chains()?;
    config::load_locked_registry()?;
    plugin::load_all_plugins()?;
    federation::start_sync_if_enabled();
    snapshot::load_snapshot().ok();
    retire::check_retirement().ok();

    tui::launch_ui()?;

    Ok(())
}
