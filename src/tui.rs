use ratatui::Terminal;
use ratatui::backend::Backend;
use ratatui::layout::{Constraint, Direction, Layout};

use crate::render::*;
use crate::state::AppState;

pub fn draw<B: Backend>(terminal: &mut Terminal<B>, state: &AppState) -> std::io::Result<()> {
    terminal.draw(|f| {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(1), Constraint::Min(0)].as_ref())
            .split(f.size());

        render_status_bar(f, chunks[0]);

        match state.mode.as_str() {
            "zen" => render_zen_journal(f, chunks[1], state),
            "mindmap" => render_mindmap(f, chunks[1], state),
            _ => render_status_bar(f, chunks[1]),
        }

        if state.show_keymap {
            render_keymap_overlay(f, chunks[1]);
        }

        if state.show_spotlight {
            render_spotlight(f, chunks[1], &state.spotlight_input);
        }

        if state.show_triage {
            let area = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Min(1), Constraint::Length(10)])
                .split(chunks[1])[1];
            render_status_bar(f, area); // Temporary triage box
        }
    })
}
