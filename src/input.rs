use crossterm::event::KeyCode;

#[derive(Debug, Clone, Copy)]
pub enum Action {
    Quit,
    ToggleHelp,
    OpenInbox,
    OpenMindmap,
}

pub fn map_key(key: KeyCode) -> Option<Action> {
    match key {
        KeyCode::Char('q') => Some(Action::Quit),
        KeyCode::Char('h') => Some(Action::ToggleHelp),
        KeyCode::Char('i') => Some(Action::OpenInbox),
        KeyCode::Char('m') => Some(Action::OpenMindmap),
        _ => None,
    }
}