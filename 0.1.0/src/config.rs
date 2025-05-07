use std::path::PathBuf;
use std::time::Duration;

#[derive(Debug, Clone, PartialEq)]
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

#[cfg(feature = "config_toml")]
pub fn load_zen_config_from_toml() -> ZenConfig {
    use std::fs;
    use toml::Value;

    let path = dirs::config_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("prismx")
        .join("config.toml");

    if let Ok(contents) = fs::read_to_string(path) {
        if let Ok(value) = contents.parse::<Value>() {
            if let Some(zen) = value.get("zen_mode") {
                let title_fade = zen
                    .get("title_fade_delay_secs")
                    .and_then(Value::as_integer)
                    .unwrap_or(2);

                let autosave = zen
                    .get("autosave_interval_secs")
                    .and_then(Value::as_integer)
                    .unwrap_or(10);

                let scratchpad_path = zen
                    .get("scratchpad_path")
                    .and_then(Value::as_str)
                    .map(PathBuf::from)
                    .unwrap_or_else(|| {
                        dirs::config_dir()
                            .unwrap_or_else(|| PathBuf::from("."))
                            .join("prismx")
                            .join("zen_scratchpad.md")
                    });

                return ZenConfig {
                    title_fade_delay: Duration::from_secs(title_fade as u64),
                    autosave_interval: Duration::from_secs(autosave as u64),
                    scratchpad_path,
                };
            }
        }
    }

    ZenConfig::default()
}