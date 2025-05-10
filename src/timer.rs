use std::time::{Instant, Duration};

pub struct PomodoroTimer {
    pub start_time: Instant,
    pub duration: Duration,
    pub running: bool,
}

impl PomodoroTimer {
    pub fn new(minutes: u64) -> Self {
        Self {
            start_time: Instant::now(),
            duration: Duration::from_secs(minutes * 60),
            running: true,
        }
    }

    pub fn is_complete(&self) -> bool {
        self.running && self.start_time.elapsed() >= self.duration
    }

    pub fn stop(&mut self) {
        self.running = false;
    }
}