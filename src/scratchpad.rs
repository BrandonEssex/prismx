// scratchpad.rs
#[derive(Debug, Clone)]
pub struct Scratchpad {
    pub content: String,
}

impl Scratchpad {
    pub fn new() -> Self {
        Self {
            content: String::new(),
        }
    }

    pub fn open(&mut self) {
        log::info!("Scratchpad opened.");
    }
}
