fn main() -> std::io::Result<()> {
    prismx::logging::init_logger();
    prismx::bootstrap::start()
}
