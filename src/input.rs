use crossterm::event::{Event, KeyCode::*, KeyEvent, KeyModifiers as KM};

use crate::actions::Action;

pub struct InputHandler;

impl InputHandler {
    pub fn handle_event(&self, event: Event) -> Option<Action> {
        if let Event::Key(KeyEvent { code, modifiers, .. }) = event {
            use KM::*;
            match (code, modifiers) {
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
        } else {
            None
        }
    }

    pub fn poll_event(&self) -> std::io::Result<Option<Event>> {
        use crossterm::event::{poll, read};
        if poll(std::time::Duration::from_millis(250))? {
            Ok(Some(read()?))
        } else {
            Ok(None)
        }
    }
}