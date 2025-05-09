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
            use KeyModifiers as KM;

            match (code, modifiers) {
                (Char('q'), KM::CONTROL) => Some(Action::Quit),
                (Char('e'), KM::CONTROL) => Some(Action::EnterEditNode),
                (Char('m'), KM::CONTROL) => Some(Action::ToggleMindmapLayout),
                (Char('c'), KM::CONTROL) => Some(Action::OpenContextMenu),
                (Char('i'), KM::CONTROL) => Some(Action::ToggleTriage),
                (Char('z'), KM::CONTROL) => Some(Action::ToggleZenMode),
                (Char('/'), KM::CONTROL) => Some(Action::ToggleShortcuts),
                (Char('l'), KM::CONTROL) => Some(Action::ToggleLogViewer),
                (Char('n'), KM::CONTROL) => Some(Action::CreateSiblingNode),
                (Tab, KM::CONTROL)       => Some(Action::CreateChildNode),
                (Char('d'), KM::CONTROL) => Some(Action::DuplicateNode),
                (Backspace, KM::CONTROL) => Some(Action::DeleteNode),
                (Char('t'), KM::CONTROL) => Some(Action::ToggleTimelineView),
                (Char('p'), KM::CONTROL) => Some(Action::ToggleMarkdownPreview),
                (Char('f'), KM::CONTROL) => Some(Action::SearchNode),
                (Char('='), KM::CONTROL) => Some(Action::ExpandNode),
                (Char('-'), KM::CONTROL) => Some(Action::CollapseNode),
                (Char('w'), KM::CONTROL) => Some(Action::SwitchWorkspace),

                (Up, KM::CONTROL)    => Some(Action::NavigateParent),
                (Down, KM::CONTROL)  => Some(Action::NavigateChild),
                (Left, KM::CONTROL)  => Some(Action::NavigateLeft),
                (Right, KM::CONTROL) => Some(Action::NavigateRight),

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