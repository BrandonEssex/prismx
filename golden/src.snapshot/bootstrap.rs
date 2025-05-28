pub fn start() -> std::io::Result<()> {
    let cfg = crate::config::load_config().unwrap_or_default();
    if let Some(level) = cfg.logging_level.as_deref() {
        std::env::set_var("PRISMX_LOG", level);
    }
    crate::logging::init_logger();
    tracing::info!("PrismX logging started");
    tracing::info!("Application bootstrap");
    crate::tui::launch_ui()
}
