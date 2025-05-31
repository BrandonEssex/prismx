use std::time::Duration;
use crossterm::event::{self, Event};

pub fn next_event(timeout: Duration) -> std::io::Result<Option<Event>> {
    if event::poll(timeout)? {
        Ok(Some(event::read()?))
    } else {
        Ok(None)
    }
}
