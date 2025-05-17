use crate::{render, spotlight, keymap, theme};
use crate::gemx::nodes::MindmapNode;
use ratatui::{
    backend::CrosstermBackend,
    Terminal,
    layout::{Layout, Constraint, Direction},
    widgets::{Block, Borders},
};
use crossterm::{
    event::{self, Event, KeyCode, KeyModifiers, KeyEvent},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, enable_raw_mode, disable_raw_mode},
};
use std::fs;
use std::io::stdout;

pub fn launch_ui() -> Result<(), Box<dyn std::error::Error>> {
    enable_raw_mode()?;
    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut zen_mode = false;
    let mut show_dashboard = true;
    let mut show_keymap = false;
    let mut show_clipboard = false;
    let mut spotlight_input = String::new();

    let root_node: MindmapNode = {
        let data = fs::read_to_string("snapshots/mindmap.json").unwrap_or_else(|_| "{}".into());
        serde_json::from_str(&data).unwrap_or_else(|_| MindmapNode::new("root", "Welcome to PrismX"))
    };

    loop {
        terminal.draw(|f| {
            let size = f.size();
            let chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints(vec![Constraint::Percentage(70), Constraint::Percentage(30)])
                .split(size);

            let main_block = Block::default().title("PrismX v10.1.0+Final").borders(Borders::ALL);
            f.render_widget(main_block, size);

            if zen_mode {
                render::render_zen_journal(f, chunks[0]);
            } else {
                render::render_mindmap(f, chunks[0], &root_node);
            }

            if show_dashboard {
                render::render_dashboard(f, chunks[1]);
            }
            if show_keymap {
                render::render_keymap_overlay(f, chunks[1]);
            }
            if show_clipboard {
                render::render_clipboard(f, chunks[1], "example copied node");
            }
            if !spotlight_input.is_empty() {
                render::render_spotlight(f, chunks[1], &spotlight_input);
            }
        })?;

        if event::poll(std::time::Duration::from_millis(100))? {
            if let Event::Key(KeyEvent { code, modifiers, .. }) = event::read()? {
                match (code, modifiers) {
                    (KeyCode::Char('q'), KeyModifiers::CONTROL) => break,
                    (KeyCode::Char('z'), KeyModifiers::CONTROL) => zen_mode = !zen_mode,
                    (KeyCode::Char('d'), KeyModifiers::CONTROL) => show_dashboard = !show_dashboard,
                    (KeyCode::Char('k'), KeyModifiers::CONTROL) => show_keymap = !show_keymap,
                    (KeyCode::Char('c'), KeyModifiers::CONTROL) => show_clipboard = !show_clipboard,
                    (KeyCode::Char('o'), KeyModifiers::CONTROL) => {
                        spotlight_input = "/theme dark".into();
                        spotlight::launch_spotlight();
                    },
                    (KeyCode::Esc, _) => spotlight_input.clear(),
                    _ => {}
                }
            }
        }
    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    Ok(())
}
