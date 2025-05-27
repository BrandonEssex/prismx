// src/zen/render.rs
use ratatui::prelude::*;
use crate::state::{AppState};
use crate::state::view::ZenViewMode;
use crate::canvas::prism::render_prism;
use crate::zen::journal::render_zen_journal;
use crate::zen::render::{render_input, render_history};
use crate::beamx::render_full_border;

/// Dispatches the correct Zen view mode renderer
pub fn render_zen<B: Backend>(f: &mut Frame<B>, area: Rect, state: &AppState) {
    match state.zen_view_mode {
        ZenViewMode::Journal => {
            render_zen_journal(f, area, state);
        }
        ZenViewMode::Classic => {
            render_classic(f, area, state);
        }
        ZenViewMode::Summary => {
            render_zen_journal(f, area, state);
        }
        ZenViewMode::Split => {
            let mid = area.width / 2;
            let left = Rect {
                x: area.x,
                y: area.y,
                width: mid,
                height: area.height,
            };
            let right = Rect {
                x: area.x + mid,
                y: area.y,
                width: area.width - mid,
                height: area.height,
            };
            render_classic(f, left, state);
            render_zen_journal(f, right, state);
        }
        ZenViewMode::Compose => {
            let tick = (std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_millis()
                / 300) as u64;
            render_compose(f, area, state, tick);
        }
    }

    render_prism(f, area);
}

/// Classic buffer-based Zen text editor
pub fn render_classic<B: Backend>(f: &mut Frame<B>, area: Rect, state: &AppState) {
    use ratatui::{text::Line, widgets::{Block, Borders, Paragraph}, style::Style};

    let lines: Vec<Line> = state
        .zen_buffer
        .iter()
        .map(|line| Line::from(line.as_str()))
        .collect();

    let para = Paragraph::new(lines)
        .block(Block::default().borders(Borders::ALL))
        .style(Style::default());

    f.render_widget(para, area);
    render_prism(f, area);
}

/// Compose mode includes input and scrollable journal view
pub fn render_compose<B: Backend>(f: &mut Frame<B>, area: Rect, state: &AppState, tick: u64) {
    use ratatui::layout::{Constraint, Direction, Layout};

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(75), Constraint::Percentage(25)].as_ref())
        .split(area);

    render_history(f, chunks[0], state);
    render_input(f, chunks[1], state, tick);

    render_full_border(f, area, &state.beam_style_for_mode(&state.mode), true, false);
}

/// One-line entry field at bottom of Compose mode
pub fn render_input<B: Backend>(f: &mut Frame<B>, area: Rect, state: &AppState, tick: u64) {
    use ratatui::widgets::{Paragraph, Block, Borders};
    use crate::ui::animate::cursor_blink;
    use chrono::Local;

    let padding = area.width / 4;
    let usable_width = area.width - padding * 2;

    let caret = cursor_blink(tick);
    let timestamp = Local::now().format("%H:%M").to_string();
    let input = format!("{} {}{}", timestamp, state.zen_draft.text, caret);

    let input_rect = Rect::new(area.x + padding, area.bottom().saturating_sub(2), usable_width, 1);
    let mut block = Block::default().borders(Borders::NONE);

    if state.zen_draft.editing.is_some() {
        block = block.borders(Borders::ALL).border_style(Style::default().fg(Color::DarkGray));
    }

    let widget = Paragraph::new(input).block(block);
    f.render_widget(widget, input_rect);
}
