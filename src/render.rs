use ratatui::{
    backend::Backend,
    layout::{Rect},
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::state::AppState;

pub fn render_status_bar<B: Backend>(f: &mut Frame<B>, area: Rect) {
    let block = Block::default()
        .borders(Borders::ALL)
        .title("Status")
        .style(Style::default().bg(Color::Black).fg(Color::White));
    f.render_widget(block, area);
}

pub fn render_zen_journal<B: Backend>(f: &mut Frame<B>, area: Rect, state: &AppState) {
    let text = state.zen_buffer.join("\n");
    let widget = Paragraph::new(text).block(Block::default().title("Zen").borders(Borders::ALL));
    f.render_widget(widget, area);
}

pub fn render_mindmap<B: Backend>(f: &mut Frame<B>, area: Rect, state: &AppState) {
    let nodes = &state.mindmap_nodes;
    let layout = Block::default().borders(Borders::ALL).title("Mindmap");
    f.render_widget(layout, area);
    for (i, node) in nodes.iter().enumerate() {
        let y = area.y + i as u16;
        if y < area.bottom() {
            let text = format!("• {}", node);
            let para = Paragraph::new(text);
            f.render_widget(para, Rect::new(area.x + 2, y, area.width - 4, 1));
        }
    }
}

pub fn render_keymap_overlay<B: Backend>(f: &mut Frame<B>, area: Rect) {
    let block = Block::default().title("Keymap").borders(Borders::ALL);
    f.render_widget(block, area);
    let content = Paragraph::new("Ctrl+K = Help\nCtrl+I = Triage\nAlt+Space = Spotlight");
    f.render_widget(content, Rect::new(area.x + 1, area.y + 1, area.width - 2, area.height - 2));
}

pub fn render_spotlight<B: Backend>(f: &mut Frame<B>, area: Rect, input: &str) {
    let block = Block::default().title("Spotlight").borders(Borders::ALL);
    let paragraph = Paragraph::new(input.to_owned());
    f.render_widget(block, area);
    f.render_widget(paragraph, Rect::new(area.x + 2, area.y + 1, area.width - 4, 1));
}
