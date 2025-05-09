use crossterm::event::{poll, read, Event, KeyCode, KeyEvent};
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
        if let Event::Key(KeyEvent { code, modifiers: _, .. }) = event {
            match code {
                KeyCode::Char('q') => Some(Action::Quit),
                KeyCode::Char('z') => Some(Action::ToggleZenMode),
                KeyCode::Char('s') => Some(Action::OpenScratchpad),
                KeyCode::Char('t') => Some(Action::ToggleTriage),
                KeyCode::Char('?') => Some(Action::ToggleShortcuts),
                KeyCode::Char('e') => Some(Action::EnterEditNode),
                KeyCode::Char('m') => Some(Action::ToggleMindmapLayout),
                KeyCode::Char('c') => Some(Action::OpenContextMenu),
                KeyCode::Esc => Some(Action::CancelEdit),
                KeyCode::Enter => Some(Action::CommitEdit),
                KeyCode::Backspace => Some(Action::PopEditChar),
                KeyCode::Right | KeyCode::Down => Some(Action::NavigateNext),
                KeyCode::Left | KeyCode::Up => Some(Action::NavigatePrev),
                KeyCode::Char(c) => Some(Action::PushEditChar(c)),
                _ => None,
            }
        } else {
            None
        }
    }
}