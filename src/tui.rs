use crate::{render, spotlight, routineforge};
use crate::gemx::nodes::MindmapNode;
use ratatui::{
    backend::CrosstermBackend,
    Terminal,
    layout::{Layout, Constraint, Direction},
    widgets::Block,
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
    let mut dashboard_on = true;
    let mut triage_on = false;
    let mut spotlight_buffer = String::new();

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

            if zen_mode {
                render::render_zen_journal(f, chunks[0]);
            } else {
                render::render_mindmap(f, chunks[0], &root_node);
            }

            if dashboard_on {
                render::render_dashboard(f, chunks[1]);
            }

            if triage_on {
                routineforge::render_triage_panel(f, chunks[1]);
            }

            if !spotlight_buffer.is_empty() {
                render::render_spotlight(f, chunks[1], &spotlight_buffer);
            }
        })?;

        if event::poll(std::time::Duration::from_millis(100))? {
            if let Event::Key(KeyEvent { code, modifiers, .. }) = event::read()? {
                match (code, modifiers) {
                    (KeyCode::Char('q'), KeyModifiers::CONTROL) => break,
                    (KeyCode::Char('z'), KeyModifiers::CONTROL) => zen_mode = !zen_mode,
                    (KeyCode::Char('d'), KeyModifiers::CONTROL) => dashboard_on = !dashboard_on,
                    (KeyCode::Char('i'), KeyModifiers::CONTROL) => triage_on = !triage_on,
                    (KeyCode::Char('o'), KeyModifiers::CONTROL) => spotlight_buffer = String::from("/"),
                    (KeyCode::Char(c), _) if spotlight_buffer.starts_with("/") => {
                        spotlight_buffer.push(c);
                    },
                    (KeyCode::Backspace, _) => {
                        spotlight_buffer.pop();
                    },
                    (KeyCode::Enter, _) => {
                        spotlight::use_command(&spotlight_buffer);
                        spotlight_buffer.clear();
                    },
                    _ => {}
                }
            }
        }
    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    Ok(())
}
