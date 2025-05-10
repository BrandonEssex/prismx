use crossterm::event::{KeyCode::*, KeyEvent};
use crate::actions::Action;

pub struct InputHandler;

impl InputHandler {
    pub fn handle_event(&self, event: KeyEvent) -> Option<Action> {
        use crossterm::event::KeyModifiers as KM;

        match (event.code, event.modifiers) {
            (Char('q'), KM::CONTROL) => Some(Action::Quit),
            (Char('e'), KM::CONTROL) => Some(Action::EnterEditNode),
            (Char('z'), KM::CONTROL) => Some(Action::ToggleZenMode),
            (Char('/'), KM::CONTROL) => Some(Action::ToggleShortcuts),
            (Char('l'), KM::CONTROL) => Some(Action::ToggleLogViewer),
            (Char('i'), KM::CONTROL) => Some(Action::ToggleTriage),
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
            (Down, KM::CONTROL)      => Some(Action::NavigateNext),
            (Up, KM::CONTROL)        => Some(Action::NavigatePrev),
            _ => None,
        }
    }

    pub fn poll_event(&self) -> std::io::Result<Option<crossterm::event::Event>> {
        use crossterm::event;
        if event::poll(std::time::Duration::from_millis(250))? {
            Ok(Some(event::read()?))
        } else {
            Ok(None)
        }
    }
}