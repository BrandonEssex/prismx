use ratatui::prelude::*;
use ratatui::text::{Line, Span};
use ratatui::style::{Color, Modifier, Style};
use ratatui::widgets::{Block, Borders};
use crate::ui::animate::shimmer;
use crate::canvas::prism::render_prism;
use crate::state::AppState;
use crate::state::view::ZenLayoutMode;
use crate::state::ZenViewMode;
use crate::zen::journal::{render_zen_journal, render_history};
use crate::beamx::render_full_border;
use crate::render::traits::{Renderable, RenderFrame};
use crate::theme::zen::zen_theme;
use super::image::render_drop_zone;

fn highlight_entry_line(text: &str) -> Line<'static> {
    let mut spans = Vec::new();
    let mut chars = text.chars().peekable();
    while let Some(c) = chars.next() {
        if c == '#' {
            let mut tag = String::from("#");
            while let Some(&ch) = chars.peek() {
                if ch.is_alphanumeric() || ch == '_' || ch == '-' {
                    tag.push(ch);
                    chars.next();
                } else {
                    break;
                }
            }
            let style = if tag.eq_ignore_ascii_case("#now") {
                Style::default().fg(Color::Green)
            } else if tag.eq_ignore_ascii_case("#done") {
                Style::default().fg(Color::Gray)
            } else {
                Style::default().fg(Color::Blue)
            };
            spans.push(Span::styled(tag, style));
        } else if c == '@' {
            let mut mention = String::from("@");
            while let Some(&ch) = chars.peek() {
                if ch.is_alphanumeric() || ch == '_' || ch == '-' {
                    mention.push(ch);
                    chars.next();
                } else {
                    break;
                }
            }
            spans.push(Span::styled(
                mention,
                Style::default().fg(Color::Yellow).add_modifier(Modifier::ITALIC),
            ));
        } else if c == '💭' || c == '🧠' || c == '📌' {
            spans.push(Span::styled(c.to_string(), Style::default().fg(Color::Magenta)));
        } else {
            spans.push(Span::raw(c.to_string()));
        }
    }
    Line::from(spans)
}

/// Dispatches the correct Zen view mode renderer
pub fn render_zen<B: Backend>(f: &mut Frame<B>, area: Rect, state: &AppState) {
    let style = state.beam_style_for_mode("zen");
    let tick = (std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis()
        / 300) as u64;
    let title_style = shimmer(style.border_color, tick);
    let block = Block::default()
        .borders(Borders::NONE)
        .title(Span::styled("Zen Mode", title_style.add_modifier(Modifier::BOLD)))
        .title_alignment(Alignment::Center);
    f.render_widget(block, area);
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
        .map(|line| highlight_entry_line(line.as_str()))
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

    let mut line = highlight_entry_line(&format!("{} {}", timestamp, visible_text));
    line.spans.push(Span::raw(caret.to_string()));

    let input_rect = Rect::new(area.x + padding, area.bottom().saturating_sub(2), usable_width, 1);
    let mut block = Block::default().borders(Borders::NONE).style(Style::default().bg(palette.background));

    if state.zen_draft.editing.is_some() {
        block = block.borders(Borders::ALL).border_style(Style::default().fg(Color::DarkGray));
    }

    let widget = Paragraph::new(line).style(Style::default().fg(palette.text).bg(palette.background)).block(block);
    f.render_widget(widget, input_rect);

    if state.enable_image_drop && state.zen_draft.text.is_empty() && state.zen_draft.editing.is_none() {
        let drop_rect = Rect::new(area.x + padding, area.y + 1, usable_width, area.height.saturating_sub(3));
        render_drop_zone(f, drop_rect);
    }
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
