pub fn start() -> std::io::Result<()> {
    crate::logging::init_logger();
    tracing::info!("[BOOT] Logger initialized");
    tracing::info!("PrismX logging started");
    tracing::info!("Application bootstrap");
    crate::tui::launch_ui()
}
