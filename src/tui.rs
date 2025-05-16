use std::io::{self, stdout};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEvent, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal, widgets::{Block, Borders, Paragraph}, layout::Rect, Frame, text::Span};

use crate::keymap::{default_keymap, Action};

pub fn launch_ui() -> Result<(), Box<dyn std::error::Error>> {
    enable_raw_mode()?;
    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    let mut zen_mode = false;

    let keymap = default_keymap();
    loop {
        terminal.draw(|f| render_ui(f, zen_mode))?;

        if event::poll(std::time::Duration::from_millis(100))? {
            if let Event::Key(KeyEvent { code, modifiers }) = event::read()? {
                match (code, modifiers) {
                    (KeyCode::Char('q'), KeyModifiers::CONTROL) => break,
                    (KeyCode::Char('z'), KeyModifiers::CONTROL) => {
                        zen_mode = !zen_mode;
                    },
                    _ => {
                        // Future handling of more actions via keymap
                    }
                }
            }
        }
    }

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;
    Ok(())
}

fn render_ui<B: ratatui::backend::Backend>(f: &mut Frame<B>, zen: bool) {
    let area = f.size();
    let block = if zen {
        Block::default()
    } else {
        Block::default().title("PrismX v10.0.0").borders(Borders::ALL)
    };
    f.render_widget(block, area);
}
