// Author: Brandon Essex
// Core plugin management system

#[derive(Default)]
pub struct PluginManager {
    pub loaded_plugins: Vec<String>, // For now, store names or IDs
}

impl PluginManager {
    pub fn handle_command(&mut self, command: String) {
        // Placeholder logic
        self.loaded_plugins.push(command);
        println!("🧩 Plugin command executed: {}", command);
    }
}
