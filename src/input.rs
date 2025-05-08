use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers};

use crate::actions::Action;

pub struct Config;

impl Config {
    pub fn map(&self, evt: Event) -> Option<Action> {
        if let Event::Key(KeyEvent { code, modifiers, .. }) = evt {
            match (code, modifiers) {
                (KeyCode::Char('q'), KeyModifiers::CONTROL) => Some(Action::Quit),
                (KeyCode::Char('d'), KeyModifiers::CONTROL) => Some(Action::ToggleDebug),
                (KeyCode::Char('z'), KeyModifiers::CONTROL) => Some(Action::ToggleZenMode),
                (KeyCode::Char('/'), KeyModifiers::CONTROL) => Some(Action::ToggleSpotlight),
                (KeyCode::Char('n'), m)
                    if m.contains(KeyModifiers::CONTROL) && m.contains(KeyModifiers::ALT) =>
                {
                    Some(Action::OpenScratchpad)
                }
                (KeyCode::Char('s'), KeyModifiers::CONTROL) => Some(Action::Ctrl('s')),
                (KeyCode::Char(c), KeyModifiers::CONTROL) => Some(Action::Ctrl(c)),
                (KeyCode::Char(c), _) => Some(Action::Input(c)),
                (KeyCode::Enter, _) => Some(Action::Submit),
                (KeyCode::Backspace, _) => Some(Action::Backspace),
                (KeyCode::Esc, _) => Some(Action::Exit),
                (KeyCode::Up, _) => Some(Action::Up),
                (KeyCode::Down, _) => Some(Action::Down),
                _ => None,
            }
        } else {
            None
        }
    }

    pub fn poll_event(&self) -> crossterm::Result<Option<Event>> {
        use crossterm::event::{poll, read};
        use std::time::Duration;

        if poll(Duration::from_millis(250))? {
            Ok(Some(read()?))
        } else {
            Ok(None)
        }
    }
}