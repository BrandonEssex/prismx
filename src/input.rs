// src/input.rs

use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers};
use crate::actions::Action;

pub struct Config;

impl Config {
    pub fn poll_event(&self) -> std::io::Result<Option<Event>> {
        if crossterm::event::poll(std::time::Duration::from_millis(200))? {
            Ok(Some(crossterm::event::read()?))
        } else {
            Ok(None)
        }
    }

    pub fn map(&self, evt: Event) -> Option<Action> {
        if let Event::Key(KeyEvent { code, modifiers, .. }) = evt {
            match (code, modifiers) {
                (KeyCode::Char('q'), KeyModifiers::CONTROL) => Some(Action::Quit),
                (KeyCode::Char('d'), KeyModifiers::CONTROL) => Some(Action::ToggleDebug),
                (KeyCode::Char('z'), KeyModifiers::CONTROL) => Some(Action::ToggleZenMode),
                (KeyCode::Char('/'), KeyModifiers::CONTROL) => Some(Action::ToggleSpotlight),
                (KeyCode::Char(c), KeyModifiers::CONTROL) => Some(Action::Ctrl(c)),
                (KeyCode::Char(c), _) => Some(Action::Char(c)),
                (KeyCode::Enter, _) => Some(Action::Enter),
                (KeyCode::Backspace, _) => Some(Action::Backspace),
                (KeyCode::Up, _) => Some(Action::MoveUp),
                (KeyCode::Down, _) => Some(Action::MoveDown),
                (KeyCode::Esc, _) => Some(Action::Exit),
                _ => None,
            }
        } else {
            None
        }
    }
}