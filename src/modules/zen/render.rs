use ratatui::prelude::*;
use crate::canvas::prism::render_prism;
use crate::state::AppState;
use crate::state::view::ZenLayoutMode;
use crate::state::ZenViewMode;
use crate::zen::journal::{render_zen_journal, render_history};
use crate::beamx::render_full_border;
use crate::render::traits::{Renderable, RenderFrame};
use crate::theme::zen::zen_theme;

/// Dispatches the correct Zen view mode renderer
pub fn render_zen<B: Backend>(f: &mut Frame<B>, area: Rect, state: &AppState) {
    match state.zen_layout_mode {
        ZenLayoutMode::Journal => {
            render_zen_journal(f, area, state);
        }
        ZenLayoutMode::Classic => {
            render_classic(f, area, state);
        }
        ZenLayoutMode::Summary => {
            render_zen_journal(f, area, state);
        }
        ZenLayoutMode::Split => {
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
        ZenLayoutMode::Dual => {
            use ratatui::widgets::Block;
            use ratatui::style::Style;

            let palette = zen_theme();
            let tick = (std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_millis()
                / 300) as u64;

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

            let bg = Block::default().style(Style::default().bg(palette.background));
            f.render_widget(bg, area);

            render_input(f, left, state, tick);
            render_history(f, right, state);
        }
        ZenLayoutMode::Compose => {
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
    use ratatui::{
        text::Line,
        widgets::{Block, Borders, Paragraph},
        style::Style,
    };
    let palette = zen_theme();

    let lines: Vec<Line> = state
        .zen_buffer
        .iter()
        .map(|line| Line::from(line.as_str()))
        .collect();

    let para = Paragraph::new(lines)
        .block(Block::default().borders(Borders::NONE).style(Style::default().bg(palette.background)))
        .style(Style::default().fg(palette.text).bg(palette.background));

    f.render_widget(para, area);
    render_prism(f, area);
}

/// Compose mode includes input and scrollable journal view
pub fn render_compose<B: Backend>(f: &mut Frame<B>, area: Rect, state: &AppState, tick: u64) {
    use ratatui::layout::{Constraint, Direction, Layout};
    use ratatui::widgets::Block;
    use ratatui::style::Style;
    let palette = zen_theme();
    let bg = Block::default().style(Style::default().bg(palette.background));
    f.render_widget(bg, area);

    // Preserve manual scroll offset when reviewing history

    if state.zen_view_mode == ZenViewMode::Write {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(75), Constraint::Percentage(25)].as_ref())
            .split(area);

        render_history(f, chunks[0], state);
        render_input(f, chunks[1], state, tick);
    } else {
        render_history(f, area, state);
    }

    render_full_border(f, area, &state.beam_style_for_mode(&state.mode), true, false);
}

/// One-line entry field at bottom of Compose mode
pub fn render_input<B: Backend>(f: &mut Frame<B>, area: Rect, state: &AppState, tick: u64) {
    use ratatui::widgets::{Paragraph, Block, Borders};
    use ratatui::style::Style;
    use crate::ui::animate::cursor_blink;
    use unicode_width::UnicodeWidthStr;
    use chrono::Local;
    let palette = zen_theme();

    let padding = area.width / 4;
    let usable_width = area.width - padding * 2;

    let caret = cursor_blink(tick);
    let timestamp = Local::now().format("%H:%M").to_string();
    let prefix = format!("{} ", timestamp);

    let prefix_width = UnicodeWidthStr::width(prefix.as_str()) as u16;
    let max_text_width = usable_width
        .saturating_sub(prefix_width + 1) as usize; // +1 for caret

    let mut visible_text = state.zen_draft.text.clone();
    while UnicodeWidthStr::width(visible_text.as_str()) > max_text_width {
        visible_text = visible_text.chars().skip(1).collect();
    }

    let input = format!("{} {}{}", timestamp, visible_text, caret);

    let input_rect = Rect::new(area.x + padding, area.bottom().saturating_sub(2), usable_width, 1);
    let mut block = Block::default().borders(Borders::NONE).style(Style::default().bg(palette.background));

    if state.zen_draft.editing.is_some() {
        block = block.borders(Borders::ALL).border_style(Style::default().fg(Color::DarkGray));
    }

    let widget = Paragraph::new(input).style(Style::default().fg(palette.text).bg(palette.background)).block(block);
    f.render_widget(widget, input_rect);
}

/// Wrapper implementing [`Renderable`] for the Zen view.
pub struct ZenView<'a> {
    pub state: &'a AppState,
}

impl<'a> ZenView<'a> {
    pub fn new(state: &'a AppState) -> Self {
        Self { state }
    }
}

impl<'a> Renderable for ZenView<'a> {
    fn render(&mut self, f: &mut RenderFrame<'_>, area: Rect) {
        render_zen(f, area, self.state);
    }
}
