## Code Changes

### 1. Add Config Struct (for toml)

```rust
#[derive(Serialize, Deserialize)]
pub struct UserSettings {
    pub auto_arrange: bool,
    pub debug_input_mode: bool,
    pub dock_layout: String, // "vertical" or "horizontal"
}
2. Load Settings at Startup
In AppState::new():

let settings_path = "config/settings.toml";
let config: UserSettings = fs::read_to_string(settings_path)
    .ok()
    .and_then(|s| toml::from_str(&s).ok())
    .unwrap_or_default();

state.auto_arrange = config.auto_arrange;
state.debug_input_mode = config.debug_input_mode;
state.favorite_dock_layout = match config.dock_layout.as_str() {
    "horizontal" => DockLayout::Horizontal,
    _ => DockLayout::Vertical,
};
3. Save Settings on Change
In toggle handler:

let config = UserSettings {
    auto_arrange: state.auto_arrange,
    debug_input_mode: state.debug_input_mode,
    dock_layout: format!("{:?}", state.favorite_dock_layout).to_lowercase(),
};

let serialized = toml::to_string(&config).unwrap();
fs::write("config/settings.toml", serialized).unwrap();
4. Add #[derive(Default)] and serde support
[dependencies]
serde = { version = "1.0", features = ["derive"] }
toml = "0.7"
