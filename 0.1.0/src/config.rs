use std::path::PathBuf;
use std::time::Duration;

#[derive(Debug, Clone)]
pub struct ZenConfig {
    pub title_fade_delay: Duration,
    pub autosave_interval: Duration,
    pub scratchpad_path: PathBuf,
}

impl ZenConfig {
    pub fn scratchpad_path(&self) -> PathBuf {
        self.scratchpad_path.clone()
    }
}

impl Default for ZenConfig {
    fn default() -> Self {
        ZenConfig {
            title_fade_delay: Duration::from_secs(2),
            autosave_interval: Duration::from_secs(10),
            scratchpad_path: dirs::config_dir()
                .unwrap_or_else(|| PathBuf::from("."))
                .join("prismx")
                .join("zen_scratchpad.md"),
        }
    }
}