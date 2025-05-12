// src/input.rs

use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers};
use crate::action::Action;

pub fn map_input_to_action(event: Event) -> Option<Action> {
    if let Event::Key(KeyEvent { code, modifiers, .. }) = event {
        match (code, modifiers) {
            (KeyCode::Char('q'), KeyModifiers::NONE) => Some(Action::Quit),
            (KeyCode::Char('h'), KeyModifiers::CONTROL) => Some(Action::ToggleHelp),
            (KeyCode::Char('z'), KeyModifiers::CONTROL) => Some(Action::ToggleZenMode),
            (KeyCode::Char('d'), KeyModifiers::CONTROL) => Some(Action::ToggleDashboard),
            (KeyCode::Char('l'), KeyModifiers::CONTROL) => Some(Action::ToggleLogView),
            (KeyCode::Char('m'), KeyModifiers::CONTROL) => Some(Action::ToggleMindmap),
            (KeyCode::Char('/'), KeyModifiers::CONTROL) => Some(Action::Custom("OpenCommand".into())),
            (KeyCode::Char('e'), KeyModifiers::CONTROL) => Some(Action::OpenExport),
            (KeyCode::Tab, KeyModifiers::NONE) => Some(Action::ToggleSidebar),
            _ => None,
        }
    } else {
        None
    }
}