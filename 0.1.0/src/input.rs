use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers};

#[derive(Debug, Clone)]
pub enum Action {
    Quit,
    Help,
    ToggleDebug,
    Redraw,
    Up,
    Down,
    Left,
    Right,
    Enter,
    Back,

    ToggleTriage,
    ToggleZenMode,
    ToggleSpotlight,
    ToggleSettings,

    NewInboxEntry,
    AssignInboxTask(String),
    SetTaskPriority(String, crate::inbox::Priority),
    TagInboxTask(String, Vec<String>),
    ArchiveTask(String),
    TriageTask(String),

    OpenScratchpad,
    Submit,
    MoveUp,
    MoveDown,
    Exit,
    Input(char),
    Backspace,
    Char(char),
    Ctrl(char),
}

pub struct Config;

impl Config {
    pub fn poll_event(&self) -> anyhow::Result<Option<Event>> {
        use crossterm::event::{poll, read};
        use std::time::Duration;

        if poll(Duration::from_millis(200))? {
            Ok(Some(read()?))
        } else {
            Ok(None)
        }
    }

    pub fn map(&self, evt: Event) -> Option<Action> {
        if let Event::Key(KeyEvent { code, modifiers, .. }) = evt {
            match (code, modifiers) {
                (KeyCode::Char('z'), KeyModifiers::CONTROL) => Some(Action::ToggleZenMode),
                (KeyCode::Char('n'), m)
                    if m.contains(KeyModifiers::CONTROL) && m.contains(KeyModifiers::ALT) =>
                {
                    Some(Action::OpenScratchpad)
                }
                (KeyCode::Char('t'), KeyModifiers::CONTROL) => Some(Action::ToggleTriage),
                (KeyCode::Char('/'), KeyModifiers::CONTROL) => Some(Action::ToggleSpotlight),
                (KeyCode::Char('q'), KeyModifiers::CONTROL) => Some(Action::Quit),
                (KeyCode::Char('s'), KeyModifiers::CONTROL) => Some(Action::Ctrl('s')),
                (KeyCode::Char(c), KeyModifiers::CONTROL) => Some(Action::Ctrl(c)),
                (KeyCode::Char(c), _) => Some(Action::Input(c)),
                (KeyCode::Enter, _) => Some(Action::Submit),
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