use crossterm::event::{poll, read, Event, KeyCode, KeyEvent, KeyModifiers};
use std::time::Duration;
use crate::actions::Action;

pub struct InputHandler;

impl InputHandler {
    pub fn poll_event(&self) -> std::io::Result<Option<Event>> {
        if poll(Duration::from_millis(200))? {
            Ok(Some(read()?))
        } else {
            Ok(None)
        }
    }

    pub fn poll_event_timeout(&self, timeout: Duration) -> std::io::Result<Option<Event>> {
        if poll(timeout)? {
            Ok(Some(read()?))
        } else {
            Ok(None)
        }
    }

    pub fn handle_event(&self, event: Event) -> Option<Action> {
        if let Event::Key(KeyEvent { code, modifiers, .. }) = event {
            use KeyCode::*;
            use KeyModifiers::*;

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

                (Esc, _) => Some(Action::CancelEdit),
                (Enter, _) => Some(Action::CommitEdit),
                (Backspace, _) => Some(Action::PopEditChar),
                (Char(c), _) => Some(Action::PushEditChar(c)),

                _ => None,
            }
        } else {
            None
        }
    }
}