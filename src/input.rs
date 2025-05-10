use crate::actions::Action;
use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers};

pub struct InputHandler;

impl InputHandler {
    pub fn handle_event(&self, event: Event) -> Option<Action> {
        if let Event::Key(KeyEvent { code, modifiers, .. }) = event {
            match (code, modifiers) {
                (KeyCode::Char('q'), KeyModifiers::CONTROL) => Some(Action::Quit),
                (KeyCode::Char('n'), KeyModifiers::CONTROL) => Some(Action::CreateSiblingNode),
                (KeyCode::Tab, KeyModifiers::CONTROL) => Some(Action::CreateChildNode),
                (KeyCode::Char('e'), KeyModifiers::CONTROL) => Some(Action::EnterEditNode),
                (KeyCode::Char('z'), KeyModifiers::CONTROL) => Some(Action::ToggleZenMode),
                (KeyCode::Char('/'), KeyModifiers::CONTROL) => Some(Action::ToggleShortcuts),
                (KeyCode::Char('l'), KeyModifiers::CONTROL) => Some(Action::ToggleLogViewer),
                (KeyCode::Enter, KeyModifiers::NONE) => Some(Action::CommitEdit),
                (KeyCode::Esc, KeyModifiers::NONE) => Some(Action::CancelEdit),
                (KeyCode::Backspace, KeyModifiers::NONE) => Some(Action::PopEditChar),
                (KeyCode::Char(c), KeyModifiers::NONE) => Some(Action::PushEditChar(c)),
                _ => None,
            }
        } else {
            None
        }
    }
}