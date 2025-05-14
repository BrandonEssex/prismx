// Author: Brandon Essex
// Maps raw input events to internal actions

use crossterm::event::{self, Event as CEvent, KeyCode, KeyEventKind};
use crate::action::Action;
use std::time::Duration;

pub fn map_input_to_action() -> Result<Option<Action>, std::io::Error> {
    if event::poll(Duration::from_millis(10))? {
        if let CEvent::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                return Ok(match key.code {
                    KeyCode::Char('q') => Some(Action::Quit),
                    KeyCode::Char('s') => Some(Action::ToggleSidebar),
                    KeyCode::Char('d') => Some(Action::SwitchView(crate::state::View::Dashboard)),
                    KeyCode::Char('m') => Some(Action::SwitchView(crate::state::View::Mindmap)),
                    KeyCode::Char('p') => Some(Action::SwitchView(crate::state::View::Plugin)),
                    KeyCode::Enter    => Some(Action::EditNode),
                    KeyCode::Tab      => Some(Action::CreateNode),
                    KeyCode::Backspace=> Some(Action::DeleteNode),
                    _ => None,
                });
            }
        }
    }
    Ok(None)
}
