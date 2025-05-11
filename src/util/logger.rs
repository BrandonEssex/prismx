use simplelog::*;
use std::fs::File;
use std::io::{Read, Write};
use std::str::FromStr;

pub struct LoggerConfig {
    pub level: String,
}

pub fn init_logger(config: &LoggerConfig) -> Result<(), Box<dyn std::error::Error>> {
    let level = LevelFilter::from_str(&config.level).unwrap_or(LevelFilter::Info);
    CombinedLogger::init(vec![
        TermLogger::new(level, Config::default(), TerminalMode::Mixed, ColorChoice::Auto),
        WriteLogger::new(level, Config::default(), File::create("logs/prismx.log")?),
    ])?;
    Ok(())
}