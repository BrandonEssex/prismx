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
        if let Event::Key(KeyEvent { code, modifiers, .. }) = event {
            match (code, modifiers) {
                (Char('q'), KM::CONTROL) => Some(Action::Quit),
                (Char('e'), KM::CONTROL) => Some(Action::EnterEditNode),
                (Char('n'), KM::CONTROL) => Some(Action::CreateSiblingNode),
                (Tab, KM::CONTROL)       => Some(Action::CreateChildNode),
                (Char('z'), KM::CONTROL) => Some(Action::ToggleZenMode),
                (Char('/'), KM::CONTROL) => Some(Action::ToggleShortcuts),
                (Char('l'), KM::CONTROL) => Some(Action::ToggleLogViewer),
                (Char('i'), KM::CONTROL) => Some(Action::ToggleTriage),
                (Char('t'), KM::CONTROL) => Some(Action::ToggleTimelineView),
                (Char('p'), KM::CONTROL) => Some(Action::ToggleMarkdownPreview),
                (Char('='), KM::CONTROL) => Some(Action::ExpandNode),
                (Char('-'), KM::CONTROL) => Some(Action::CollapseNode),
                (Left, KM::CONTROL)      => Some(Action::NavigateLeft),
                (Right, KM::CONTROL)     => Some(Action::NavigateRight),
                (Up, KM::CONTROL)        => Some(Action::NavigatePrev),
                (Down, KM::CONTROL)      => Some(Action::NavigateNext),
                (Char('d'), KM::CONTROL) => Some(Action::DuplicateNode),
                (Backspace, KM::CONTROL) => Some(Action::DeleteNode),
                (Enter, _) => Some(Action::CommitEdit),
                (Esc, _) => Some(Action::CancelEdit),
                (Backspace, _) => Some(Action::PopEditChar),
                (Char(c), _) => Some(Action::PushEditChar(c)),
                _ => None,
            }
        } else {
            None
        }
    }
}