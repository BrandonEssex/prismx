use ratatui::{
    prelude::*,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
};

use chrono::Datelike;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::state::{AppState, ZenSyntax, ZenTheme, ZenMode};
use crate::state::view::ZenViewMode;
use crate::zen::journal::{extract_tags, render_history};
use crate::zen::utils::highlight_tags_line;
use crate::beamx::render_full_border;
use crate::ui::beamx::{BeamX, BeamXStyle, BeamXMode, BeamXAnimationMode};
use crate::render::traits::Renderable;
use crate::canvas::prism::render_prism;


const TOP_BAR_HEIGHT: u16 = 5;

pub struct ZenRenderer<'a> {
    pub state: &'a AppState,
}

impl<'a> ZenRenderer<'a> {
    pub fn new(state: &'a AppState) -> Self {
        Self { state }
    }
}

impl<'a> Renderable for ZenRenderer<'a> {
    fn render_frame<B: Backend>(&mut self, f: &mut Frame<B>, area: Rect) {
        render_zen_journal(f, area, self.state);
    }

    fn tick(&mut self) {}
}

pub fn render_zen_journal<B: Backend>(f: &mut Frame<B>, area: Rect, state: &AppState) {
    render_history(f, area, state);
}

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
            let tick = (SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_millis()
                / 300) as u64;
            render_compose(f, area, state, tick);
        }
    }

    render_prism(f, area);
}


fn render_compose<B: Backend>(f: &mut Frame<B>, area: Rect, state: &AppState, tick: u64) {
    use ratatui::layout::{Constraint, Direction, Layout};
    use crate::render::zen::render_input;

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(75), Constraint::Percentage(25)].as_ref())
        .split(area);

    render_history(f, chunks[0], state);
    render_input(f, chunks[1], state, tick);

    render_full_border(f, area, &state.beam_style_for_mode(&state.mode), true, false);
}

fn render_input<B: Backend>(f: &mut Frame<B>, area: Rect, state: &AppState, tick: u64) {
    use crate::ui::animate::cursor_blink;

    let padding = area.width / 4;
    let usable_width = area.width - padding * 2;

    let caret = cursor_blink(tick);
    let timestamp = chrono::Local::now().format("%H:%M").to_string();
    let input = format!("{} {}{}", timestamp, state.zen_draft.text, caret);

    let input_rect = Rect::new(area.x + padding, area.bottom().saturating_sub(2), usable_width, 1);
    let mut block = Block::default().borders(Borders::NONE);

    if state.zen_draft.editing.is_some() {
        block = Block::default().borders(Borders::ALL).border_style(Style::default().fg(Color::DarkGray));
    }

    let widget = Paragraph::new(input).block(block);
    f.render_widget(widget, input_rect);
}

fn render_review<B: Backend>(f: &mut Frame<B>, area: Rect, state: &AppState) {
    use crate::ui::animate::fade_line;
    use crate::config::theme::ThemeConfig;

    let breathe = ThemeConfig::load().zen_breathe();
    let padding = area.width / 4;
    let usable_width = area.width - padding * 2;
    let mut y = area.y + TOP_BAR_HEIGHT;

    for entry in state.filtered_journal_entries().into_iter().rev() {
        let mut lines = vec![];
        let ts = entry.timestamp.format("%I:%M %p").to_string();
        lines.push(Line::from(vec![
            Span::raw("\u{1F551} "),
            Span::styled(ts, Style::default().fg(Color::DarkGray).add_modifier(Modifier::DIM)),
        ]));

        for l in entry.text.lines() {
            lines.push(highlight_tags_line(l));
        }

        lines.push(Line::from(Span::styled("────────────", Style::default().fg(Color::Gray).add_modifier(Modifier::DIM))));

        if breathe {
            let age = (chrono::Local::now() - entry.timestamp).num_milliseconds() as u128;
            for line in lines.iter_mut() {
                fade_line(line, age, 150);
            }
        }

        let rect = Rect::new(area.x + padding, y, usable_width, lines.len() as u16);
        let p = Paragraph::new(lines).block(Block::default().borders(Borders::NONE));
        f.render_widget(p, rect);
        y = y.saturating_add(3);
        if y > area.bottom() {
            break;
        }
    }

    render_full_border(f, area, &state.beam_style_for_mode(&state.mode), true, false);
}

fn render_top_icon<B: Backend>(f: &mut Frame<B>, area: Rect, state: &AppState, tick: u64) {
    if !state.zen_icon_enabled {
        let glyph = state.zen_icon_glyph.as_deref().unwrap_or("✦");
        let style = Style::default().fg(Color::Gray);
        let x = area.right().saturating_sub(glyph.len() as u16 + 1);
        f.render_widget(Paragraph::new(glyph).style(style), Rect::new(x, area.y + 1, glyph.len() as u16, 1));
        return;
    }

    if let Some(glyph) = &state.zen_icon_glyph {
        let style = Style::default().fg(Color::Magenta);
        let x = area.right().saturating_sub(glyph.len() as u16 + 1);
        f.render_widget(Paragraph::new(glyph.as_str()).style(style), Rect::new(x, area.y + 1, glyph.len() as u16, 1));
    } else {
        let mut bx_style = BeamXStyle::from(BeamXMode::Zen);
        let theme = state.beam_style_for_mode(&state.mode);
        bx_style.border_color = theme.border_color;
        bx_style.status_color = theme.status_color;
        bx_style.prism_color = theme.prism_color;
        let beamx = BeamX {
            tick,
            enabled: true,
            mode: BeamXMode::Zen,
            style: bx_style,
            animation: BeamXAnimationMode::PulseEntryRadiate,
        };
        beamx.render(f, area);
    }
}
