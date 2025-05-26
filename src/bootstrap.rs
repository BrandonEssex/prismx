pub fn start() -> std::io::Result<()> {
    crate::logging::init_logger();
    crate::tui::launch_ui()
}
