use crate::action::Action;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

pub fn map_key_event(event: KeyEvent) -> Option<Action> {
    match (event.code, event.modifiers) {
        (KeyCode::Char('q'), KeyModifiers::CONTROL) => Some(Action::Quit),
        (KeyCode::Char('n'), KeyModifiers::CONTROL) => Some(Action::CreateSiblingNode),
        (KeyCode::Char('z'), KeyModifiers::CONTROL) => Some(Action::ToggleZenMode),
        (KeyCode::Tab, KeyModifiers::CONTROL) => Some(Action::CreateChildNode),
        _ => None,
    }
}