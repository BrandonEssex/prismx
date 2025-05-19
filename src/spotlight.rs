use std::collections::HashMap;

pub struct Spotlight {
    pub input: String,
    pub commands: HashMap<String, Box<dyn FnMut()>>,
}

impl Spotlight {
    pub fn new() -> Self {
        Self {
            input: String::new(),
            commands: HashMap::new(),
        }
    }

    pub fn register_command<F>(&mut self, name: &str, handler: F)
    where
        F: FnMut() + 'static,
    {
        self.commands.insert(name.to_string(), Box::new(handler));
    }

    pub fn execute(&mut self) {
        let trimmed = self.input.trim();
        if let Some(cmd) = self.commands.get_mut(trimmed) {
            cmd();
        } else {
            // Ignore unknown command (remove spam)
        }
    }
}
