use std::io::Result;
use crossterm::event::{poll, read, Event, KeyCode, KeyEvent};

use crate::actions::Action;

pub struct InputHandler;

impl InputHandler {
    pub fn poll_event(&self) -> Result<Option<Event>> {
        if poll(std::time::Duration::from_millis(200))? {
            Ok(Some(read()?))
        } else {
            Ok(None)
        }
    }

    pub fn handle_event(&self, event: Event) -> Option<Action> {
        if let Event::Key(KeyEvent { code, .. }) = event {
            match code {
                KeyCode::Char('q') => Some(Action::Quit),
                KeyCode::Char('z') => Some(Action::ToggleZenMode),
                KeyCode::Char('s') => Some(Action::OpenScratchpad),
                KeyCode::Char('t') => Some(Action::ToggleTriage),
                _ => None,
            }
        } else {
            None
        }
    }
}
