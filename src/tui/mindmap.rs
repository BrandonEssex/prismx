use ratatui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders, canvas::{Canvas, Context}},
    Terminal,
    text::Line,
};
use crossterm::event::{self, Event, KeyCode, MouseEvent, MouseEventKind};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, io};
use tokio::sync::Mutex;
use lazy_static::lazy_static;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Position {
    pub x: f64,
    pub y: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Node {
    pub id: u64,
    pub title: String,
    pub position: Position,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Mindmap {
    pub nodes: HashMap<u64, Node>,
    pub next_id: u64,
}

impl Mindmap {
    pub fn add_node(&mut self, title: &str, position: Position) -> u64 {
        let id = self.next_id;
        self.nodes.insert(id, Node {
            id,
            title: title.to_string(),
            position,
        });
        self.next_id += 1;
        id
    }

    pub fn remove_node(&mut self, id: u64) -> bool {
        self.nodes.remove(&id).is_some()
    }

    pub fn move_node(&mut self, id: u64, new_position: Position) -> bool {
        if let Some(node) = self.nodes.get_mut(&id) {
            node.position = new_position;
            true
        } else {
            false
        }
    }
}

#[derive(Debug)]
pub struct MindmapState {
    pub mindmap: Mindmap,
    pub undo_redo: crate::util::undo_redo::ActionStack,
}

impl MindmapState {
    pub fn new() -> Self {
        Self {
            mindmap: Mindmap::default(),
            undo_redo: crate::util::undo_redo::ActionStack::new(),
        }
    }
}

lazy_static! {
    pub static ref STATE: Mutex<MindmapState> = Mutex::new(MindmapState::new());
}

pub async fn run_tui_loop() -> Result<(), Box<dyn std::error::Error>> {
    let backend = ratatui::backend::CrosstermBackend::new(io::stdout());
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;

    loop {
        terminal.draw(|f| {
            let size = f.size();
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Percentage(100)])
                .split(size);

            let canvas = Canvas::default()
                .block(Block::default().borders(Borders::ALL).title("Mindmap"))
                .x_bounds([0.0, 100.0])
                .y_bounds([0.0, 100.0])
                .paint(draw_nodes);

            f.render_widget(canvas, chunks[0]);
        })?;

        if event::poll(std::time::Duration::from_millis(100))? {
            match event::read()? {
                Event::Key(key_event) => {
                    if key_event.code == KeyCode::Char('q') {
                        break;
                    }
                }
                Event::Mouse(mouse_event) => handle_mouse(mouse_event).await?,
                _ => {}
            }
        }
    }

    terminal.clear()?;
    Ok(())
}

fn draw_nodes(ctx: &mut Context) {
    let state = STATE.blocking_lock();
    for node in state.mindmap.nodes.values() {
        ctx.print(node.position.x, node.position.y, Line::from(node.title.clone()));
    }
}

async fn handle_mouse(event: MouseEvent) -> Result<(), Box<dyn std::error::Error>> {
    if let MouseEventKind::Down(_) = event.kind {
        let mut state = STATE.lock().await;
        state.mindmap.add_node("New", Position {
            x: event.column as f64,
            y: event.row as f64,
        });
    }
    Ok(())
}