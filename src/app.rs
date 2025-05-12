// src/app.rs

use crate::screen::Screen;
use crossterm::event::{self, Event as CEvent, KeyCode};
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;
use std::io::{self, stdout};
use std::time::{Duration, Instant};

pub fn run() -> io::Result<()> {
    Screen::<CrosstermBackend<std::io::Stdout>>::enter_alt_screen()?;

    let backend = CrosstermBackend::new(stdout());
    let terminal = Terminal::new(backend)?;

    let mut screen = Screen::new(terminal);
    let mut last_tick = Instant::now();
    let tick_rate = Duration::from_millis(250);

    loop {
        screen.draw()?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if event::poll(timeout)? {
            if let CEvent::Key(key) = event::read()? {
                if key.code == KeyCode::Char('q') {
                    break;
                }
                screen.handle_event(CEvent::Key(key));
            }
        }

        if last_tick.elapsed() >= tick_rate {
            last_tick = Instant::now();
            // Add tick-based updates here if needed
        }
    }

    Screen::<CrosstermBackend<std::io::Stdout>>::exit_alt_screen()?;
    Ok(())
}