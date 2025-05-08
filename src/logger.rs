use log::LevelFilter;
use std::str::FromStr;

pub fn init_logger(config: &crate::config::Config) {
    let level = LevelFilter::from_str(&config.level).unwrap_or(LevelFilter::Info);
    env_logger::Builder::new()
        .filter_level(level)
        .init();
}
