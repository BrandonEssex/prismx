use ratatui::Terminal;
use ratatui::backend::Backend;
use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui::layout::Rect;
use crossterm::event::{read, Event, KeyCode};

use crate::state::AppState;

pub struct Screen {
    state: AppState,
}

impl Screen {
    pub fn new() -> Self {
        Self {
            state: AppState::default(),
        }
    }

    pub fn run<B: Backend>(&mut self, terminal: &mut Terminal<B>) -> Result<(), Box<dyn std::error::Error>> {
        let mut show_help = false;

        loop {
            terminal.draw(|f| {
                let area: Rect = f.size();
                let block = if show_help {
                    Paragraph::new("Help Panel\nPress 'q' to quit.")
                } else {
                    Paragraph::new("PrismX Running...\nPress 'h' for help.")
                }
                .block(Block::default().title("PrismX").borders(Borders::ALL));
                f.render_widget(block, area);
            })?;

            if let Event::Key(key) = read()? {
                match key.code {
                    KeyCode::Char('q') => break,
                    KeyCode::Char('h') => show_help = !show_help,
                    _ => {}
                }
            }
        }

        Ok(())
    }
}