use crate::{spotlight, keymap, theme};
use ratatui::{
    backend::CrosstermBackend,
    Terminal,
    layout::{Layout, Constraint, Direction},
    widgets::{Block, Borders},
};
use crossterm::{
    event::{self, Event, KeyCode, KeyModifiers, KeyEvent},
    execute, terminal::{EnterAlternateScreen, LeaveAlternateScreen, enable_raw_mode, disable_raw_mode},
};
use std::io::stdout;

pub fn launch_ui() -> Result<(), Box<dyn std::error::Error>> {
    enable_raw_mode()?;
    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    let mut zen_mode = false;

    loop {
        terminal.draw(|f| {
            let size = f.size();
            let block = if zen_mode {
                Block::default()
            } else {
                Block::default().title("PrismX v10.1.0").borders(Borders::ALL)
            };
            f.render_widget(block, size);
        })?;

        if event::poll(std::time::Duration::from_millis(100))? {
            if let Event::Key(KeyEvent { code, modifiers, .. }) = event::read()? {
                match (code, modifiers) {
                    (KeyCode::Char('q'), KeyModifiers::CONTROL) => break,
                    (KeyCode::Char('z'), KeyModifiers::CONTROL) => zen_mode = !zen_mode,
                    (KeyCode::Char('o'), KeyModifiers::CONTROL) => spotlight::launch_spotlight(),
                    (KeyCode::Char('t'), KeyModifiers::CONTROL) => theme::toggle_theme(),
                    (KeyCode::Char('k'), KeyModifiers::CONTROL) => keymap::show_overlay(),
                    _ => {}
                }
            }
        }
    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    Ok(())
}
