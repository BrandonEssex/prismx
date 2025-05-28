pub fn start() -> std::io::Result<()> {
    let cfg = crate::config::load_config();
    if let Some(level) = cfg.log_level.as_deref() {
        std::env::set_var("PRISMX_LOG", level);
    }
    if let Some(theme) = cfg.theme.as_deref() {
        crate::theme::set_theme(theme);
    }
    crate::logger::init_logger();
    tracing::info!("PrismX logging started");
    tracing::info!("Application bootstrap");
    crate::tui::launch_ui()
}
