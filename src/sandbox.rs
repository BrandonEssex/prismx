use std::collections::HashMap;

#[derive(Default)]
pub struct Sandbox {
    pub environment: HashMap<String, String>,
}

impl Sandbox {
    pub fn new() -> Self {
        Self { environment: HashMap::new() }
    }

    pub fn set(&mut self, key: &str, value: &str) {
        self.environment.insert(key.to_string(), value.to_string());
    }

    pub fn get(&self, key: &str) -> Option<&String> {
        self.environment.get(key)
    }
}
