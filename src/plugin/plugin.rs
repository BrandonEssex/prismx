#[derive(Default)]
pub struct PluginManager {
    pub loaded_plugins: Vec<String>,
}

impl PluginManager {
    pub fn handle_command(&mut self, command: String) {
        self.loaded_plugins.push(command.clone());
        println!("🧩 Plugin command executed: {}", command);
    }
}
