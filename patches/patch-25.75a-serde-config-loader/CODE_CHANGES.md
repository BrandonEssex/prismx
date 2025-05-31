## Code Changes

- Create `config.rs` for user settings
- Parse config via Serde (default: TOML)
- Fallback to embedded defaults if file not found
- Versioned schema support via `config.version = "1"`
