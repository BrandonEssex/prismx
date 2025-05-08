use log::{info, warn, error};
use std::env;

pub fn init_logging() {
    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", "info");
    }

    let _ = env_logger::builder()
        .format_timestamp_secs()
        .is_test(env::var("RUST_LOG").ok().map(|s| s.contains("test")).unwrap_or(false))
        .try_init();
}