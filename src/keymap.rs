use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum Command {
    Quit,
    NewNode,
    CutNode,
    CopyNode,
    PasteNode,
    OpenSettings,
    OpenSpotlight,
    OpenDashboard,
    OpenMindmap,
    OpenZenMode,
    ExportMindTrace,
    ToggleShortcuts,
}

pub fn get_command(event: KeyEvent) -> Option<Command> {
    match (event.code, event.modifiers) {
        (KeyCode::Char('q'), _) => Some(Command::Quit),
        (KeyCode::Char('n'), KeyModifiers::CONTROL) => Some(Command::NewNode),
        (KeyCode::Char('x'), KeyModifiers::CONTROL) => Some(Command::CutNode),
        (KeyCode::Char('c'), KeyModifiers::CONTROL) => Some(Command::CopyNode),
        (KeyCode::Char('v'), KeyModifiers::CONTROL) => Some(Command::PasteNode),
        (KeyCode::Char('.'), KeyModifiers::CONTROL) => Some(Command::OpenSettings),
        (KeyCode::Char(' '), KeyModifiers::ALT) => Some(Command::OpenSpotlight),
        (KeyCode::Char('d'), KeyModifiers::CONTROL) => Some(Command::OpenDashboard),
        (KeyCode::Char('m'), KeyModifiers::CONTROL) => Some(Command::OpenMindmap),
        (KeyCode::Char('z'), KeyModifiers::CONTROL) => Some(Command::OpenZenMode),
        (KeyCode::Char('e'), KeyModifiers::CONTROL) => Some(Command::ExportMindTrace),
        (KeyCode::Char('s'), KeyModifiers::CONTROL) => Some(Command::ToggleShortcuts),
        _ => None,
    }
}
