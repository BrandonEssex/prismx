use ratatui::Terminal;
use ratatui::backend::Backend;
use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui::layout::{Layout, Constraint, Direction, Rect};
use crossterm::event::{read, Event, KeyCode, KeyEventKind};

use crate::state::{AppState, View, SidebarView};
use crate::input::{map_key, Action};
use crate::ui::sidebar::render_sidebar;
use crate::ui::help_overlay::render_help_overlay;
use crate::ui::command_bar::render_command_bar;

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
        loop {
            terminal.draw(|f| {
                let vertical_chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints(
                        if self.state.in_command_mode {
                            vec![Constraint::Min(0), Constraint::Length(3)]
                        } else {
                            vec![Constraint::Percentage(100)]
                        }
                    )
                    .split(f.size());

                let main_area = vertical_chunks[0];
                let command_area = if self.state.in_command_mode {
                    Some(vertical_chunks[1])
                } else {
                    None
                };

                let chunks = Layout::default()
                    .direction(Direction::Horizontal)
                    .constraints(
                        if self.state.sidebar_view != SidebarView::None {
                            vec![Constraint::Percentage(70), Constraint::Percentage(30)]
                        } else {
                            vec![Constraint::Percentage(100)]
                        }
                    )
                    .split(main_area);

                let main_chunk = chunks[0];
                let sidebar_chunk = if self.state.sidebar_view != SidebarView::None {
                    Some(chunks[1])
                } else {
                    None
                };

                match self.state.view {
                    View::HelpOverlay => {
                        render_help_overlay(f, main_chunk);
                    }
                    _ => {
                        let content = match self.state.view {
                            View::Help => "Help View\nPress 'q' to quit.",
                            View::Inbox => "Inbox View\nPress 'h' for help.",
                            View::Mindmap => "Mindmap View\nPress 'h' for help.",
                            View::Dashboard => "Dashboard Panel",
                            _ => "PrismX Running...",
                        };

                        let block = Paragraph::new(content)
                            .block(Block::default().title("PrismX").borders(Borders::ALL));

                        f.render_widget(block, main_chunk);
                    }
                }

                if let Some(side_area) = sidebar_chunk {
                    render_sidebar(f, side_area, &self.state.sidebar_view);
                }

                if let Some(cmd_area) = command_area {
                    render_command_bar(f, cmd_area, &self.state.command_input);
                }
            })?;

            if let Event::Key(key) = read()? {
                if self.state.in_command_mode {
                    match key.code {
                        KeyCode::Esc => {
                            self.state.in_command_mode = false;
                            self.state.command_input.clear();
                        }
                        KeyCode::Enter => {
                            match self.state.command_input.trim() {
                                "mindmap" => self.state.view = View::Mindmap,
                                "inbox" => self.state.view = View::Inbox,
                                "dashboard" => self.state.view = View::Dashboard,
                                "help" => self.state.view = View::HelpOverlay,
                                _ => {}
                            }
                            self.state.command_input.clear();
                            self.state.in_command_mode = false;
                        }
                        KeyCode::Char(c) => {
                            self.state.command_input.push(c);
                        }
                        KeyCode::Backspace => {
                            self.state.command_input.pop();
                        }
                        _ => {}
                    }
                } else {
                    if let Some(action) = map_key(key.code) {
                        match action {
                            Action::Quit => break,
                            Action::ToggleHelp => {
                                self.state.view = if self.state.view == View::Help {
                                    View::Mindmap
                                } else {
                                    View::Help
                                };
                            }
                            Action::OpenInbox => self.state.view = View::Inbox,
                            Action::OpenMindmap => self.state.view = View::Mindmap,
                            Action::NextSidebarTab => {
                                self.state.sidebar_view = match self.state.sidebar_view {
                                    SidebarView::Meta => SidebarView::Outline,
                                    SidebarView::Outline => SidebarView::Tags,
                                    SidebarView::Tags => SidebarView::None,
                                    SidebarView::None => SidebarView::Meta,
                                }
                            }
                            Action::HideSidebar => {
                                self.state.sidebar_view = SidebarView::None;
                            }
                        }
                    }

                    if key.code == KeyCode::Char('?') {
                        self.state.view = View::HelpOverlay;
                    }

                    if key.code == KeyCode::Char('/') {
                        self.state.in_command_mode = true;
                        self.state.command_input.clear();
                    }
                }
            }
        }

        Ok(())
    }
}