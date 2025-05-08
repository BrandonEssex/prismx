use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Favorites {
    pub data: HashMap<String, bool>,
}

impl Favorites {
    pub fn new() -> Self {
        Favorites {
            data: HashMap::new(),
        }
    }

    pub fn toggle(&mut self, uid: &str) {
        let entry = self.data.entry(uid.to_string()).or_insert(false);
        *entry = !*entry;
    }

    pub fn is_favorite(&self, uid: &str) -> bool {
        *self.data.get(uid).unwrap_or(&false)
    }
}