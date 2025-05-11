use ratatui::Terminal;
use ratatui::backend::Backend;
use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui::layout::Rect;
use crossterm::event::{read, Event, KeyCode};

use crate::state::{AppState, View};
use crate::input::{map_key, Action};

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
                let area: Rect = f.size();
                let content = match self.state.view {
                    View::Help => "Help View\nPress 'q' to quit.",
                    View::Inbox => "Inbox View\nPress 'h' for help.",
                    View::Mindmap => "Mindmap View\nPress 'h' for help.",
                };
                let block = Paragraph::new(content)
                    .block(Block::default().title("PrismX").borders(Borders::ALL));
                f.render_widget(block, area);
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
                    }
                }
            }
        }

        Ok(())
    }
}