use crate::{dashboard, spotlight, keymap, theme};
use ratatui::{
    backend::CrosstermBackend,
    Terminal,
    layout::{Layout, Constraint, Direction},
    widgets::{Block, Borders, Paragraph},
    text::{Span, Spans},
};
use crossterm::{
    event::{self, Event, KeyCode, KeyModifiers, KeyEvent},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, enable_raw_mode, disable_raw_mode},
};
use std::io::stdout;

pub fn launch_ui() -> Result<(), Box<dyn std::error::Error>> {
    enable_raw_mode()?;
    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    let mut zen_mode = false;
    let mut show_dashboard = false;

    loop {
        terminal.draw(|f| {
            let size = f.size();

            // Unified layout to satisfy type check for both branches
            let chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints(
                    if show_dashboard {
                        vec![Constraint::Percentage(70), Constraint::Percentage(30)]
                    } else {
                        vec![Constraint::Percentage(100)]
                    }
                )
                .split(size);

            let main_block = if zen_mode {
                Block::default()
            } else {
                Block::default().title("PrismX v10.1.0+Final").borders(Borders::ALL)
            };
            f.render_widget(main_block, chunks[0]);

            if show_dashboard && chunks.len() > 1 {
                let panel = dashboard::render_panel();
                f.render_widget(panel, chunks[1]);
            }
        })?;

        if event::poll(std::time::Duration::from_millis(100))? {
            if let Event::Key(KeyEvent { code, modifiers, .. }) = event::read()? {
                match (code, modifiers) {
                    (KeyCode::Char('q'), KeyModifiers::CONTROL) => break,
                    (KeyCode::Char('z'), KeyModifiers::CONTROL) => zen_mode = !zen_mode,
                    (KeyCode::Char('d'), KeyModifiers::CONTROL) => show_dashboard = !show_dashboard,
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
