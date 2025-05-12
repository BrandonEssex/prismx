// src/app.rs

use crate::screen::Screen;
use crate::node_tree::NodeTree;
use crossterm::event::{self, Event as CEvent, KeyCode};
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;
use std::io::{self, stdout};
use std::time::{Duration, Instant};

pub fn run() -> io::Result<()> {
    Screen::<CrosstermBackend<std::io::Stdout>>::enter_alt_screen()?;

    let stdout = stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut screen = Screen::new(terminal);
    let tree = NodeTree::with_mock_data(); // Ensure visible mindmap nodes
    let mut last_tick = Instant::now();
    let tick_rate = Duration::from_millis(250);

    loop {
        screen.draw_with_tree(&tree)?;

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
        }
    }

    Screen::<CrosstermBackend<std::io::Stdout>>::exit_alt_screen()?;
    Ok(())
}