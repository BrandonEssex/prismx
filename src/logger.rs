use anyhow::Result;
use env_logger::Env;

pub fn init_logging() -> Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    log::info!("Logger initialized successfully.");
    Ok(())
}