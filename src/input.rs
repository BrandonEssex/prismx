use crossterm::event::{poll, read, Event, KeyCode, KeyEvent, KeyModifiers};
use std::time::Duration;
use crate::actions::Action;

pub struct InputHandler;

impl InputHandler {
    pub fn poll_event(&self) -> std::io::Result<Option<Event>> {
        if poll(Duration::from_millis(200))? {
            Ok(Some(read()?))
        } else {
            Ok(None)
        }
    }

    pub fn poll_event_timeout(&self, timeout: Duration) -> std::io::Result<Option<Event>> {
        if poll(timeout)? {
            Ok(Some(read()?))
        } else {
            Ok(None)
        }
    }

    pub fn handle_event(&self, event: Event) -> Option<Action> {
        if let Event::Key(KeyEvent { code, modifiers, .. }) = event {
            match (code, modifiers) {
                (KeyCode::Char('q'), KeyModifiers::CONTROL) => Some(Action::Quit),
                (KeyCode::Char('e'), KeyModifiers::CONTROL) => Some(Action::EnterEditNode),
                (KeyCode::Char('m'), KeyModifiers::CONTROL) => Some(Action::ToggleMindmapLayout),
                (KeyCode::Char('c'), KeyModifiers::CONTROL) => Some(Action::OpenContextMenu),
                (KeyCode::Char('i'), KeyModifiers::CONTROL) => Some(Action::ToggleTriage),
                (KeyCode::Char('z'), KeyModifiers::CONTROL) => Some(Action::ToggleZenMode),
                (KeyCode::Char('/'), KeyModifiers::CONTROL) => Some(Action::ToggleShortcuts),
                (KeyCode::Esc, _) => Some(Action::CancelEdit),
                (KeyCode::Enter, _) => Some(Action::CommitEdit),
                (KeyCode::Backspace, _) => Some(Action::PopEditChar),
                (KeyCode::Right | KeyCode::Down, _) => Some(Action::NavigateNext),
                (KeyCode::Left | KeyCode::Up, _) => Some(Action::NavigatePrev),
                (KeyCode::Char(c), _) => Some(Action::PushEditChar(c)),
                _ => None,
            }
        } else {
            None
        }
    }
}