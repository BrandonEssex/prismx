use ratatui::{
    backend::CrosstermBackend,
    Terminal,
};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::io;
use crate::dashboard::render_dashboard;
use crate::mindmap::{render_mindmap, MindmapState};
use crate::mindtrace::MindTrace;
use crate::keymap::{get_command, Command};
use crate::icon::render_prismx_icon;

#[derive(Debug, Clone, Copy)]
enum AppState {
    Dashboard,
    Mindmap,
    Zen,
}

pub fn run_ui() -> Result<(), Box<dyn std::error::Error>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app_state = AppState::Dashboard;
    let mut mindmap_state = MindmapState::new();
    let mut mindtrace = MindTrace::new();

    loop {
        terminal.draw(|f| {
            let area = f.size();
            match app_state {
                AppState::Dashboard => {
                    render_dashboard(f);
                    let icon_area = ratatui::layout::Rect {
                        x: area.width.saturating_sub(10),
                        y: 0,
                        width: 9,
                        height: 3,
                    };
                    render_prismx_icon(f, icon_area);
                }
                AppState::Mindmap => {
                    render_mindmap(f, area, &mindtrace, &mindmap_state);
                }
                AppState::Zen => {
                    use ratatui::{text::Line, widgets::{Paragraph, Block, Borders}};
                    let block = Paragraph::new(Line::from("Zen Mode (coming soon)"))
                        .block(Block::default().title("Zen").borders(Borders::ALL));
                    f.render_widget(block, area);
                }
            }
        })?;

        if event::poll(std::time::Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                match app_state {
                    AppState::Mindmap => match key.code {
                        KeyCode::Char(c) => mindmap_state.input.push(c),
                        KeyCode::Backspace => {
                            mindmap_state.input.pop();
                        }
                        KeyCode::Enter => {
                            if !mindmap_state.input.trim().is_empty() {
                                mindtrace.add_node(&mindmap_state.selected_id, &mindmap_state.input);
                                mindmap_state.input.clear();
                            }
                        }
                        KeyCode::Esc => {
                            app_state = AppState::Dashboard;
                        }
                        _ => {}
                    },
                    _ => {
                        if let Some(cmd) = get_command(key) {
                            match cmd {
                                Command::Quit => break,
                                Command::NewNode => {
                                    app_state = AppState::Mindmap;
                                    mindmap_state.input.clear();
                                }
                                Command::CutNode => {}
                                Command::OpenSettings => {}
                                Command::OpenSpotlight => {}
                                Command::OpenDashboard => app_state = AppState::Dashboard,
                                Command::OpenMindmap => app_state = AppState::Mindmap,
                                Command::OpenZenMode => app_state = AppState::Zen,
                            }
                        }
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
