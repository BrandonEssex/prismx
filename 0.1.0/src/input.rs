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
}

pub struct Config;

impl Config {
    pub fn map(&self, evt: Event) -> Option<Action> {
        if let Event::Key(KeyEvent { code, modifiers }) = evt {
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
                _ => None,
            }
        } else {
            None
        }
    }
}