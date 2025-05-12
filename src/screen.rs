// src/screen.rs

use crate::action::Action;
use crate::input::map_input_to_action;
use crate::state::{AppState, SidebarView, View};
use crate::node_tree::NodeTree;
use ratatui::backend::Backend;
use ratatui::Terminal;
use std::io;

use crossterm::event::{self, DisableMouseCapture, EnableMouseCapture, Event as CEvent};
use crossterm::execute;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen};

use crate::ui::draw::draw;

pub struct Screen<B: Backend> {
    terminal: Terminal<B>,
    pub state: AppState,
}

impl<B: Backend> Screen<B> {
    pub fn new(terminal: Terminal<B>) -> Self {
        Self {
            terminal,
            state: AppState::default(),
        }
    }

    pub fn draw_with_tree(&mut self, tree: &NodeTree) -> io::Result<()> {
        self.terminal.draw(|f| draw(f, &self.state, tree))?;
        Ok(())
    }

    pub fn handle_event(&mut self, event: CEvent) {
        if let Some(action) = map_input_to_action(event) {
            self.handle_action(action);
        }
    }

    pub fn handle_action(&mut self, action: Action) {
        match action {
            Action::Quit => {}
            Action::ToggleHelp => {
                self.state.sidebar = if self.state.sidebar == SidebarView::Help {
                    SidebarView::Hidden
                } else {
                    SidebarView::Help
                };
            }
            Action::ToggleSidebar => {
                self.state.sidebar = if self.state.sidebar == SidebarView::Hidden {
                    SidebarView::Help
                } else {
                    SidebarView::Hidden
                };
            }
            Action::ToggleZenMode => self.state.view = View::Zen,
            Action::ToggleDashboard => self.state.view = View::Dashboard,
            Action::ToggleLogView => self.state.view = View::Log,
            Action::ToggleMindmap => self.state.view = View::Mindmap,
            Action::OpenExport => self.state.view = View::Export,
            Action::Redraw => {}
            Action::Custom(_) => {}
        }
    }

    pub fn enter_alt_screen() -> io::Result<()> {
        enable_raw_mode()?;
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
        Ok(())
    }

    pub fn exit_alt_screen() -> io::Result<()> {
        disable_raw_mode()?;
        let mut stdout = io::stdout();
        execute!(stdout, LeaveAlternateScreen, DisableMouseCapture)?;
        Ok(())
    }

    pub fn clear_screen(&mut self) -> io::Result<()> {
        self.terminal.clear()
    }

    pub fn handle_action(&mut self, action: Action) {
        match action {
            Action::Quit => {}
            Action::ToggleHelp => {
                self.state.sidebar = if self.state.sidebar == SidebarView::Help {
                    SidebarView::Hidden
                } else {
                    SidebarView::Help
                };
            }
            Action::ToggleSidebar => {
                self.state.sidebar = if self.state.sidebar == SidebarView::Hidden {
                    SidebarView::Help
                } else {
                    SidebarView::Hidden
                };
            }
            Action::ToggleZenMode => self.state.view = View::Zen,
            Action::ToggleDashboard => self.state.view = View::Dashboard,
            Action::ToggleLogView => self.state.view = View::Log,
            Action::ToggleMindmap => self.state.view = View::Mindmap,
            Action::OpenExport => self.state.view = View::Export,
            Action::Escape => {
                if self.state.view == View::Zen {
                    self.state.view = View::Dashboard;
                } else {
                    self.state.sidebar = SidebarView::Hidden;
                }
            }
            Action::ToggleCommandBar => {
                // future: open command buffer
            }
            Action::Redraw => {}
            Action::Custom(_) => {}
        }
    }
}