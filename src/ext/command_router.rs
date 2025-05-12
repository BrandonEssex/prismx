// src/ext/command_router.rs

use crate::action::Action;
use std::collections::HashMap;

#[derive(Debug, Default)]
pub struct CommandRouter {
    bindings: HashMap<String, Action>,
}

impl CommandRouter {
    pub fn new() -> Self {
        let mut bindings = HashMap::new();
        bindings.insert("zen".to_string(), Action::ToggleZenMode);
        bindings.insert("dashboard".to_string(), Action::ToggleDashboard);
        bindings.insert("log".to_string(), Action::ToggleLogView);
        bindings.insert("mindmap".to_string(), Action::ToggleMindmap);
        bindings.insert("export".to_string(), Action::OpenExport);
        bindings.insert("help".to_string(), Action::ToggleHelp);

        Self { bindings }
    }

    pub fn resolve(&self, command: &str) -> Option<Action> {
        self.bindings.get(command).cloned()
    }

    pub fn register(&mut self, keyword: &str, action: Action) {
        self.bindings.insert(keyword.to_string(), action);
    }
}