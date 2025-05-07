use simplelog::*;
use std::fs::File;

pub fn init_logging() {
    let _ = CombinedLogger::init(vec![
        TermLogger::new(LevelFilter::Info, Config::default(), TerminalMode::Mixed, ColorChoice::Auto),
        WriteLogger::new(LevelFilter::Debug, Config::default(), File::create("logs/prismx.log").unwrap()),
    ]);
}