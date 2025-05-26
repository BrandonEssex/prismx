use std::sync::OnceLock;
use std::time::{Duration, Instant};

static START: OnceLock<Instant> = OnceLock::new();

fn start() -> Instant {
    *START.get_or_init(Instant::now)
}

/// Return the current tick based on the given interval in milliseconds.
pub fn tick(interval_ms: u64) -> u64 {
    start().elapsed().as_millis() as u64 / interval_ms
}

/// Access the elapsed time since the engine was first used.
pub fn elapsed() -> Duration {
    start().elapsed()
}
