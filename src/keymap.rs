// src/keymap.rs

use crossterm::event::{KeyCode, KeyModifiers};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct KeyCombo {
    pub code: KeyCode,
    pub modifiers: KeyModifiers,
}

impl KeyCombo {
    pub fn new(code: KeyCode, modifiers: KeyModifiers) -> Self {
        Self { code, modifiers }
    }
}

#[derive(Debug, Clone)]
pub struct Keymap {
    pub bindings: HashMap<KeyCombo, String>,
}

impl Keymap {
    pub fn new() -> Self {
        let mut bindings = HashMap::new();
        bindings.insert(KeyCombo::new(KeyCode::Char('h'), KeyModifiers::CONTROL), "help".into());
        bindings.insert(KeyCombo::new(KeyCode::Char('z'), KeyModifiers::CONTROL), "zen".into());
        bindings.insert(KeyCombo::new(KeyCode::Char('d'), KeyModifiers::CONTROL), "dashboard".into());
        bindings.insert(KeyCombo::new(KeyCode::Char('l'), KeyModifiers::CONTROL), "log".into());
        bindings.insert(KeyCombo::new(KeyCode::Char('m'), KeyModifiers::CONTROL), "mindmap".into());
        bindings.insert(KeyCombo::new(KeyCode::Char('e'), KeyModifiers::CONTROL), "export".into());
        Self { bindings }
    }

    pub fn get_command(&self, combo: &KeyCombo) -> Option<&String> {
        self.bindings.get(combo)
    }
}