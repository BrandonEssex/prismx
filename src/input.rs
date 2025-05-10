use crate::action::Action;
use crossterm::event::{KeyCode::*, KeyEvent, KeyModifiers};

pub fn map_key_event(event: KeyEvent) -> Option<Action> {
    use KeyCode::*;
    use KeyModifiers::*;

    match (event.code, event.modifiers) {
        (Char('q'), CONTROL) => Some(Action::Quit),
        (Char('e'), CONTROL) => Some(Action::EnterEditNode),
        (Char('n'), CONTROL) => Some(Action::CreateSiblingNode),
        (Tab, CONTROL)       => Some(Action::CreateChildNode),
        (Char('z'), CONTROL) => Some(Action::ToggleZenMode),
        (Char('/'), CONTROL) => Some(Action::ToggleShortcuts),
        (Char('l'), CONTROL) => Some(Action::ToggleLogViewer),
        (Char('i'), CONTROL) => Some(Action::ToggleTriage),
        (Char('t'), CONTROL) => Some(Action::ToggleTimelineView),
        (Char('p'), CONTROL) => Some(Action::ToggleMarkdownPreview),
        (Char('='), CONTROL) => Some(Action::ExpandNode),
        (Char('-'), CONTROL) => Some(Action::CollapseNode),
        (Left, CONTROL)      => Some(Action::NavigateLeft),
        (Right, CONTROL)     => Some(Action::NavigateRight),
        (Up, CONTROL)        => Some(Action::NavigatePrev),
        (Down, CONTROL)      => Some(Action::NavigateNext),
        (Char('d'), CONTROL) => Some(Action::DuplicateNode),
        (Backspace, CONTROL) => Some(Action::DeleteNode),
        _ => None,
    }
}