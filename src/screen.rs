use ratatui::Terminal;
use ratatui::backend::Backend;
use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui::layout::{Layout, Constraint, Direction, Rect};
use crossterm::event::{read, Event, KeyCode};

use crate::state::{AppState, View, SidebarView};
use crate::input::{map_key, Action};
use crate::ui::sidebar::render_sidebar;

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
                let chunks = Layout::default()
                    .direction(Direction::Horizontal)
                    .constraints(
                        if self.state.sidebar_view != SidebarView::None {
                            vec![Constraint::Percentage(70), Constraint::Percentage(30)]
                        } else {
                            vec![Constraint::Percentage(100)]
                        }
                    )
                    .split(f.size());

                let main_chunk = chunks[0];
                let sidebar_chunk = if self.state.sidebar_view != SidebarView::None {
                    Some(chunks[1])
                } else {
                    None
                };

                let content = match self.state.view {
                    View::Help => "Help View\nPress 'q' to quit.",
                    View::Inbox => "Inbox View\nPress 'h' for help.",
                    View::Mindmap => "Mindmap View\nPress 'h' for help.",
                    View::Dashboard => "Dashboard Panel",
                    View::HelpOverlay => "Help Overlay",
                };

                let block = Paragraph::new(content)
                    .block(Block::default().title("PrismX").borders(Borders::ALL));

                f.render_widget(block, main_chunk);

                if let Some(side_area) = sidebar_chunk {
                    render_sidebar(f, side_area, &self.state.sidebar_view);
                }
            })?;

            if let Event::Key(key) = read()? {
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
            }
        }

        Ok(())
    }
}