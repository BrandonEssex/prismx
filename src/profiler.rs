use std::time::{Duration, Instant};

pub struct Profiler {
    pub label: String,
    pub start: Instant,
}

impl Profiler {
    pub fn new(label: &str) -> Self {
        Self {
            label: label.to_string(),
            start: Instant::now(),
        }
    }

    pub fn end(&self) -> Duration {
        self.start.elapsed()
    }

    pub fn log(&self) {
        let elapsed = self.end();
        println!("[Profiler] {} took {:?}", self.label, elapsed);
    }
}