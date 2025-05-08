use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders, canvas::{Canvas, Context}},
    Terminal,
};
use crossterm::event::{self, Event, KeyCode, MouseEvent, MouseEventKind};
use crate::util::errors::MindmapError;
use crate::util::logger;
use crate::util::undo_redo::{ActionStack, NodeAction};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, io};
use lazy_static::lazy_static;
use tokio::sync::Mutex;

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
        self.nodes.insert(id, Node { id, title: title.to_string(), position });
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
    pub undo_redo: ActionStack,
}

impl MindmapState {
    pub fn new() -> Self {
        Self {
            mindmap: Mindmap::default(),
            undo_redo: ActionStack::new(),
        }
    }
}

lazy_static! {
    static ref STATE: Mutex<MindmapState> = Mutex::new(MindmapState::new());
}

pub async fn run_tui_loop() -> Result<(), MindmapError> {
    let backend = CrosstermBackend::new(io::stdout());
    let mut terminal = Terminal::new(backend).map_err(|e| MindmapError::TUIError(e.to_string()))?;
    terminal.clear().map_err(|e| MindmapError::TUIError(e.to_string()))?;

    loop {
        terminal.draw(|f| {
            let size = f.size();
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Percentage(100)])
                .split(size);

            let canvas = Canvas::default()
                .block(Block::default().borders(Borders::ALL).title("PrismX Mindmap"))
                .x_bounds([0.0, 100.0])
                .y_bounds([0.0, 100.0])
                .paint(draw_nodes);

            f.render_widget(canvas, chunks[0]);
        }).map_err(|e| MindmapError::TUIError(e.to_string()))?;

        if event::poll(std::time::Duration::from_millis(100)).unwrap_or(false) {
            match event::read().map_err(|e| MindmapError::TUIError(e.to_string()))? {
                Event::Key(key_event) => {
                    match key_event.code {
                        KeyCode::Char('q') => break,
                        KeyCode::Char('n') => add_node_interactive().await?,
                        KeyCode::Char('d') => delete_selected_node().await?,
                        KeyCode::Char('z') if key_event.modifiers.contains(event::KeyModifiers::CONTROL) => {
                            undo_action().await?;
                        }
                        KeyCode::Char('y') if key_event.modifiers.contains(event::KeyModifiers::CONTROL) => {
                            redo_action().await?;
                        }
                        _ => {}
                    }
                }
                Event::Mouse(mouse_event) => handle_mouse(mouse_event).await?,
                _ => {}
            }
        }
    }

    terminal.clear().map_err(|e| MindmapError::TUIError(e.to_string()))?;
    Ok(())
}

fn draw_nodes(ctx: &mut Context) {
    let state = STATE.blocking_lock();
    for node in state.mindmap.nodes.values() {
        ctx.print(node.position.x, node.position.y, &node.title);
    }
}

async fn add_node_interactive() -> Result<(), MindmapError> {
    let position = Position { x: 50.0, y: 50.0 };
    let mut state = STATE.lock().await;
    let id = state.mindmap.add_node("New Node", position.clone());
    state.undo_redo.push(NodeAction::Create(id));
    logger::log_action("Node added interactively", id);
    Ok(())
}

async fn delete_selected_node() -> Result<(), MindmapError> {
    let node_id = {
        let state = STATE.lock().await;
        state.mindmap.next_id - 1
    };
    let mut state = STATE.lock().await;
    if state.mindmap.remove_node(node_id) {
        state.undo_redo.push(NodeAction::Delete(node_id));
        logger::log_action("Node deleted", node_id);
    }
    Ok(())
}

async fn handle_mouse(event: MouseEvent) -> Result<(), MindmapError> {
    match event.kind {
        MouseEventKind::Down(_) => {
            let position = Position { x: event.column as f64, y: event.row as f64 };
            let mut state = STATE.lock().await;
            let id = state.mindmap.add_node("Mouse Node", position.clone());
            state.undo_redo.push(NodeAction::Create(id));
            logger::log_action("Node added via mouse", id);
        }
        _ => {}
    }
    Ok(())
}

async fn undo_action() -> Result<(), MindmapError> {
    let mut state = STATE.lock().await;
    if let Some(action) = state.undo_redo.undo() {
        match action {
            NodeAction::Create(id) => {
                state.mindmap.remove_node(id);
                logger::log_action("Undo node creation", id);
            }
            NodeAction::Delete(id) => {
                state.mindmap.add_node("Restored Node", Position { x: 50.0, y: 50.0 });
                logger::log_action("Undo node deletion", id);
            }
        }
    }
    Ok(())
}

async fn redo_action() -> Result<(), MindmapError> {
    let mut state = STATE.lock().await;
    if let Some(action) = state.undo_redo.redo() {
        match action {
            NodeAction::Create(id) => {
                state.mindmap.add_node("Redone Node", Position { x: 50.0, y: 50.0 });
                logger::log_action("Redo node creation", id);
            }
            NodeAction::Delete(id) => {
                state.mindmap.remove_node(id);
                logger::log_action("Redo node deletion", id);
            }
        }
    }
    Ok(())
}