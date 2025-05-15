use ratatui::{backend::CrosstermBackend, Terminal};
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
use crate::spotlight::render_spotlight;
use crate::clipboard::{copy_node, cut_node, paste_node};
use crate::export_engine::export_to_json;

#[derive(Debug, Clone, Copy)]
enum AppState {
    Dashboard,
    Mindmap,
    Zen,
    Spotlight,
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
    let mut clipboard = None;
    let mut show_shortcuts = true;

    loop {
        terminal.draw(|f| {
            let area = f.size();
            match app_state {
                AppState::Dashboard => {
                    render_dashboard(f, show_shortcuts);
                    let icon_area = ratatui::layout::Rect {
                        x: area.width.saturating_sub(11),
                        y: 0,
                        width: 10,
                        height: 3,
                    };
                    render_prismx_icon(f, icon_area);
                }
                AppState::Mindmap => {
                    render_mindmap(f, area, &mindtrace, &mindmap_state);
                }
                AppState::Zen => {
                    use ratatui::widgets::{Block, Borders, Paragraph};
                    use ratatui::text::Line;
                    let zen = Paragraph::new(Line::from("Zen Mode Active"))
                        .block(Block::default().title("Zen").borders(Borders::ALL));
                    f.render_widget(zen, area);
                }
                AppState::Spotlight => {
                    render_spotlight(f, area);
                }
            }
        })?;

        if event::poll(std::time::Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                match app_state {
                    AppState::Mindmap => match key.code {
                        KeyCode::Char(c) => {
                            mindmap_state.input.insert(mindmap_state.cursor_position, c);
                            mindmap_state.cursor_position += 1;
                        }
                        KeyCode::Backspace => {
                            if mindmap_state.cursor_position > 0 {
                                mindmap_state.cursor_position -= 1;
                                mindmap_state.input.remove(mindmap_state.cursor_position);
                            }
                        }
                        KeyCode::Left => {
                            if mindmap_state.cursor_position > 0 {
                                mindmap_state.cursor_position -= 1;
                            }
                        }
                        KeyCode::Right => {
                            if mindmap_state.cursor_position < mindmap_state.input.len() {
                                mindmap_state.cursor_position += 1;
                            }
                        }
                        KeyCode::Enter => {
                            if !mindmap_state.input.trim().is_empty() {
                                mindtrace.add_node(&mindmap_state.selected_id, &mindmap_state.input);
                                mindmap_state.input.clear();
                                mindmap_state.cursor_position = 0;
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
                                    mindmap_state.cursor_position = 0;
                                }
                                Command::CutNode => {
                                    clipboard = cut_node(&mut mindtrace, &mindmap_state.selected_id);
                                }
                                Command::CopyNode => {
                                    clipboard = copy_node(&mindtrace, &mindmap_state.selected_id);
                                }
                                Command::PasteNode => {
                                    if let Some(data) = clipboard.clone() {
                                        mindtrace.add_node(&mindmap_state.selected_id, &data);
                                    }
                                }
                                Command::OpenSettings => {}
                                Command::OpenSpotlight => app_state = AppState::Spotlight,
                                Command::OpenDashboard => app_state = AppState::Dashboard,
                                Command::OpenMindmap => app_state = AppState::Mindmap,
                                Command::OpenZenMode => app_state = AppState::Zen,
                                Command::ExportMindTrace => {
                                    let _ = export_to_json(&mindtrace, "mindtrace_export.json");
                                }
                                Command::ToggleShortcuts => {
                                    show_shortcuts = !show_shortcuts;
                                }
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
