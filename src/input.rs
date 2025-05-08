use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers};

use crate::actions::Action;

pub struct Config;

impl Config {
    pub fn poll_event(&self) -> std::io::Result<Option<Event>> {
        use crossterm::event;
        if event::poll(std::time::Duration::from_millis(100))? {
            Ok(Some(event::read()?))
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
                (KeyCode::Up, _) => Some(Action::Up),
                (KeyCode::Down, _) => Some(Action::Down),
                (KeyCode::Left, _) => Some(Action::Left),
                (KeyCode::Right, _) => Some(Action::Right),
                (KeyCode::Enter, _) => Some(Action::Enter),
                (KeyCode::Esc, _) => Some(Action::Back),
                (KeyCode::Char(c), KeyModifiers::CONTROL) => Some(Action::Ctrl(c)),
                (KeyCode::Char(c), _) => Some(Action::Char(c)),
                _ => None,
            }
        } else {
            None
        }
    }
}