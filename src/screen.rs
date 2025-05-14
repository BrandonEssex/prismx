use crate::action::Action;
use crate::input::map_input_to_action;
use crate::state::{AppState, View};
use crate::ui::draw::draw;

use crossterm::event::{self, Event as CEvent, KeyCode};
use ratatui::backend::Backend;
use ratatui::Terminal;
use std::io;
use std::time::{Duration};

pub struct Screen<B: Backend> {
    pub terminal: Terminal<B>,
    pub app_state: AppState,
}

impl<B: Backend> Screen<B> {
    pub fn new(terminal: Terminal<B>, app_state: AppState) -> Self {
        Self { terminal, app_state }
    }

    pub fn run(&mut self) -> io::Result<()> {
        loop {
            self.terminal.draw(|f| draw(f, &self.app_state))?;

            if event::poll(Duration::from_millis(250))? {
                if let CEvent::Key(key) = event::read()? {
                    match key.code {
                        KeyCode::Char('q') => {
                            self.app_state.should_quit = true;
                        }
                        _ => {
                            if let Some(action) = map_input_to_action()? {
                                self.handle_action(action);
                            }
                        }
                    }
                }
            }

            if self.app_state.should_quit {
                break;
            }
        }
        Ok(())
    }

    fn handle_action(&mut self, action: Action) {
        match action {
            Action::Quit => self.app_state.should_quit = true,
            Action::ToggleSidebar => self.app_state.sidebar_view.toggle(),
            Action::SwitchView(view) => self.app_state.current_view = view,
            Action::PluginCommand(cmd) => self.app_state.plugin_manager.handle_command(cmd),
            Action::EditNode => self.app_state.node_tree.begin_editing_selected(),
            Action::CreateNode => self.app_state.node_tree.create_child_node(),
            Action::DeleteNode => self.app_state.node_tree.delete_selected(),
        }
    }
}
