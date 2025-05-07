use std::fs;
use std::path::PathBuf;
use std::time::{Duration, Instant};

const AUTOSAVE_INTERVAL: Duration = Duration::from_secs(10);
const FALLBACK_TEMPLATE: &str = "# Zen Scratchpad\n\nStart typing here...";

pub struct Scratchpad {
    buffer: String,
    path: PathBuf,
    last_saved: Instant,
    modified: bool,
    autosave_interval: Duration,
}

impl Scratchpad {
    pub fn new(path: PathBuf) -> Self {
        let buffer = Self::load_or_default(&path);
        Self {
            buffer,
            path,
            last_saved: Instant::now(),
            modified: false,
            autosave_interval: AUTOSAVE_INTERVAL,
        }
    }

    fn load_or_default(path: &PathBuf) -> String {
        match fs::read_to_string(path) {
            Ok(content) => content,
            Err(_) => {
                if let Some(parent) = path.parent() {
                    let _ = fs::create_dir_all(parent);
                }
                let _ = fs::write(path, FALLBACK_TEMPLATE);
                FALLBACK_TEMPLATE.to_string()
            }
        }
    }

    pub fn get_buffer(&self) -> &str {
        &self.buffer
    }

    pub fn update(&mut self, new_content: String) {
        if new_content != self.buffer {
            self.buffer = new_content;
            self.modified = true;
        }
    }

    pub fn autosave_if_needed(&mut self) {
        if self.modified && self.last_saved.elapsed() > self.autosave_interval {
            if let Err(e) = fs::write(&self.path, &self.buffer) {
                eprintln!("Failed to autosave scratchpad: {}", e);
            } else {
                self.last_saved = Instant::now();
                self.modified = false;
            }
        }
    }

    pub fn force_save(&mut self) {
        if let Err(e) = fs::write(&self.path, &self.buffer) {
            eprintln!("Scratchpad manual save failed: {}", e);
        } else {
            self.last_saved = Instant::now();
            self.modified = false;
        }
    }

    pub fn set_autosave_interval(&mut self, interval: Duration) {
        self.autosave_interval = interval;
    }
}