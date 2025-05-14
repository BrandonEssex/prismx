// Author: Brandon Essex
// Placeholder for PrismX config system

use std::fs;

pub fn load_config() -> Result<String, std::io::Error> {
    let raw = fs::read_to_string("config/plugin.json")?;
    Ok(raw)
}
