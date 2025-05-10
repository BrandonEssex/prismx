#[derive(Debug, Default, Clone)]
pub struct Scratchpad {
    pub content: String,
}

impl Scratchpad {
    pub fn open(&mut self) {
        // Placeholder: could load or toggle editor view
        self.content.push_str("\n[opened scratchpad]");
    }
}