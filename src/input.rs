use crossterm::event::{Event, KeyCode::*, KeyEvent, KeyModifiers as KM};
use crate::actions::Action;

pub struct InputHandler;

impl InputHandler {
    pub fn poll_event(&self) -> std::io::Result<Option<Event>> {
        if crossterm::event::poll(std::time::Duration::from_millis(250))? {
            Ok(Some(crossterm::event::read()?))
        } else {
            Ok(None)
        }
    }

    pub fn handle_event(&self, event: Event) -> Option<Action> {
        if let Event::Key(KeyEvent { code, modifiers }) = event {
            use KM::*;
            match (code, modifiers) {
                (Char('q'), CONTROL) => Some(Action::Quit),
                (Char('e'), CONTROL) => Some(Action::EnterEditNode),
                (Char('m'), CONTROL) => Some(Action::ToggleMindmapLayout),
                (Char('c'), CONTROL) => Some(Action::OpenContextMenu),
                (Char('i'), CONTROL) => Some(Action::ToggleTriage),
                (Char('z'), CONTROL) => Some(Action::ToggleZenMode),
                (Char('/'), CONTROL) => Some(Action::ToggleShortcuts),
                (Char('l'), CONTROL) => Some(Action::ToggleLogViewer),
                (Char('n'), CONTROL) => Some(Action::CreateSiblingNode),
                (Tab, CONTROL)       => Some(Action::CreateChildNode),
                (Char('d'), CONTROL | SHIFT) => Some(Action::DuplicateNode),
                (Backspace, CONTROL) => Some(Action::DeleteNode),
                (Char('t'), CONTROL) => Some(Action::ToggleTimelineView),
                (Char('p'), CONTROL) => Some(Action::ToggleMarkdownPreview),
                (Char('f'), CONTROL) => Some(Action::SearchNode),
                (Char('='), CONTROL) => Some(Action::ExpandNode),
                (Char('-'), CONTROL) => Some(Action::CollapseNode),
                (Char('w'), CONTROL) => Some(Action::SwitchWorkspace),
                (Char('t'), CONTROL | ALT) => Some(Action::ToggleTagFilterMenu),
                (Up, CONTROL)    => Some(Action::NavigateParent),
                (Down, CONTROL)  => Some(Action::NavigateChild),
                (Left, CONTROL)  => Some(Action::NavigateLeft),
                (Right, CONTROL) => Some(Action::NavigateRight),
                (Backspace, _) => Some(Action::PopEditChar),
                (Enter, _) => Some(Action::CommitEdit),
                (Esc, _) => Some(Action::CancelEdit),
                (Char(c), _) => Some(Action::PushEditChar(c)),
                _ => None,
            }
        } else {
            None
        }
    }
}