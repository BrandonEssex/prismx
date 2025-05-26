use ratatui::{prelude::*, widgets::{Block, Borders, Paragraph}, style::Modifier};
use crate::state::{AppState, ZenSyntax, ZenTheme, ZenJournalView, ZenViewMode};
use crate::zen::journal::extract_tags;
use crate::beamx::render_full_border;
use crate::ui::beamx::{BeamX, BeamXStyle, BeamXMode, BeamXAnimationMode};
use crate::render::traits::Renderable;

const TOP_BAR_HEIGHT: u16 = 5;

use std::time::{SystemTime, UNIX_EPOCH};

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
    let mut style = state.beam_style_for_mode(&state.mode);
    if let ZenTheme::DarkGray = state.zen_theme {
        style.border_color = Color::DarkGray;
    }

    let total_height = area.height as usize;

    if total_height < 4 || area.width < 10 {
        return;
    }

    // Frame tick for simple animations
    let tick = (SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis()
        / 300) as u64;

    // Background pulse effect
    let bg_color = match state.zen_theme {
        ZenTheme::DarkGray => match tick % 20 {
            0..=9 => Color::Rgb(18, 18, 18),
            _ => Color::Rgb(12, 12, 12),
        },
        ZenTheme::Light => match tick % 20 {
            0..=9 => Color::Rgb(240, 240, 240),
            _ => Color::Rgb(230, 230, 230),
        },
        ZenTheme::HighContrast => Color::Black,
    };
    let bg = Block::default().style(Style::default().bg(bg_color));
    f.render_widget(bg, area);

    match state.zen_journal_view {
        ZenJournalView::Compose => render_compose(f, area, state, tick),
        ZenJournalView::Review => render_review(f, area, state),
    }
    render_top_icon(f, area, state, tick);
}

fn render_compose<B: Backend>(f: &mut Frame<B>, area: Rect, state: &AppState, tick: u64) {
    use ratatui::layout::{Constraint, Direction, Layout};

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(75), Constraint::Percentage(25)].as_ref())
        .split(area);

    render_history(f, chunks[0], state);
    render_input(f, chunks[1], state, tick);

    render_full_border(f, area, &state.beam_style_for_mode(&state.mode), true, false);
}

fn render_history<B: Backend>(f: &mut Frame<B>, area: Rect, state: &AppState) {
    let padding = area.width / 4;
    let usable_width = area.width - padding * 2;

    if state.zen_journal_entries.is_empty() {
        let msg = Paragraph::new("No journal entries yet.")
            .alignment(Alignment::Center);
        let rect = Rect::new(area.x + padding, area.y + area.height / 2, usable_width, 1);
        f.render_widget(msg, rect);
        return;
    }

    let entries = state.filtered_journal_entries();
    let mut blocks: Vec<Vec<Line>> = Vec::new();
    let mut current_date = String::new();
    for entry in entries.iter() {
        if matches!(state.zen_view_mode, ZenViewMode::Summary) {
            let d = entry.timestamp.format("%b %d, %Y").to_string();
            if current_date != d {
                blocks.push(vec![Line::from(Span::styled(d.clone(), Style::default().fg(Color::Magenta)))]);
                current_date = d;
            }
        }

        let tags = extract_tags(&entry.text);
        let tag_line = if tags.is_empty() { None } else { Some(tags.join(" ")) };
        let mut lines: Vec<Line> = Vec::new();
        let ts = entry.timestamp.format("%b %d, %Y – %-I:%M%p").to_string();
        lines.push(Line::from(Span::styled(ts, Style::default().fg(Color::DarkGray).add_modifier(Modifier::DIM))));
        if let Some(t) = tag_line {
            lines.push(highlight_tags_line(&t));
        }
        for l in entry.text.lines() { lines.push(highlight_tags_line(l)); }
        lines.push(Line::from(Span::styled("────────────", Style::default().fg(Color::Gray).add_modifier(Modifier::DIM))));
        blocks.push(lines);
    }

    let mut y = area.bottom();
    for block in blocks.into_iter().rev() {
        let h = block.len() as u16;
        if y < area.y + h { break; }
        y -= h;
        let rect = Rect::new(area.x + padding, y, usable_width, h);
        let widget = Paragraph::new(block).block(Block::default().borders(Borders::NONE));
        f.render_widget(widget, rect);
        if y > area.y { y -= 1; }
    }
}

fn render_input<B: Backend>(f: &mut Frame<B>, area: Rect, state: &AppState, tick: u64) {
    let padding = area.width / 4;
    let usable_width = area.width - padding * 2;
    use crate::ui::animate::cursor_blink;
    let caret = cursor_blink(tick);
    let timestamp = chrono::Local::now().format("%H:%M").to_string();
    let input = format!("{} {}{}", timestamp, state.zen_compose_input, caret);
    let input_rect = Rect::new(area.x + padding, area.bottom().saturating_sub(2), usable_width, 1);
    let widget = Paragraph::new(input).block(Block::default().borders(Borders::NONE));
    f.render_widget(widget, input_rect);
}

fn render_review<B: Backend>(f: &mut Frame<B>, area: Rect, state: &AppState) {
    use ratatui::text::{Span, Line};
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
        let rect = Rect::new(area.x + padding, y, usable_width, lines.len() as u16);
        let p = Paragraph::new(lines).block(Block::default().borders(Borders::NONE));
        f.render_widget(p, rect);
        y = y.saturating_add(3);
        if y > area.bottom() { break; }
    }
    render_full_border(f, area, &state.beam_style_for_mode(&state.mode), true, false);
}

fn render_top_icon<B: Backend>(f: &mut Frame<B>, area: Rect, state: &AppState, tick: u64) {
    use ratatui::style::Modifier;
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

fn parse_markdown_line(input: &str) -> Line {
    use ratatui::text::{Span, Line};
    use ratatui::style::Modifier;

    if input.starts_with("### ") {
        return Line::from(Span::styled(&input[4..], Style::default().add_modifier(Modifier::ITALIC)));
    } else if input.starts_with("## ") {
        return Line::from(Span::from(&input[3..]));
    } else if input.starts_with("# ") {
        return Line::from(Span::from(&input[2..]));
    } else if input.starts_with("- ") || input.starts_with("* ") {
        return Line::from(vec![
            Span::raw("• "),
            Span::raw(&input[2..]),
        ]);
    }

    let mut spans = vec![];
    let mut chars = input.chars().peekable();
    while let Some(c) = chars.next() {
        if c == '*' && chars.peek() == Some(&'*') {
            chars.next();
            let mut bold = String::new();
            while let Some(&next) = chars.peek() {
                if next == '*' {
                    chars.next();
                    if chars.peek() == Some(&'*') {
                        chars.next();
                        break;
                    }
                }
                bold.push(next);
                chars.next();
            }
            spans.push(Span::raw(bold));
        } else if c == '_' {
            let mut italic = String::new();
            while let Some(&next) = chars.peek() {
                if next == '_' {
                    chars.next();
                    break;
                }
                italic.push(next);
                chars.next();
            }
            spans.push(Span::styled(italic, Style::default().add_modifier(Modifier::ITALIC)));
        } else {
            spans.push(Span::raw(c.to_string()));
        }
    }

    Line::from(spans)
}

fn parse_rust_line(input: &str) -> Line {
    use ratatui::text::{Span, Line};
    let keywords = ["fn", "let", "mut", "pub", "struct", "impl", "match", "if", "else", "use", "mod"];
    let mut spans = Vec::new();
    for token in input.split_whitespace() {
        if keywords.contains(&token) {
            spans.push(Span::styled(token, Style::default().fg(Color::Cyan)));
        } else {
            spans.push(Span::raw(token));
        }
        spans.push(Span::raw(" "));
    }
    Line::from(spans)
}

fn parse_shell_line(input: &str) -> Line {
    use ratatui::text::{Span, Line};
    let cmds = ["cd", "ls", "echo", "sudo", "cat", "cp", "mv"];
    let mut spans = Vec::new();
    for token in input.split_whitespace() {
        if cmds.contains(&token) {
            spans.push(Span::styled(token, Style::default().fg(Color::Green)));
        } else if token.starts_with('#') {
            spans.push(Span::styled(token, Style::default().fg(Color::DarkGray)));
        } else {
            spans.push(Span::raw(token));
        }
        spans.push(Span::raw(" "));
    }
    Line::from(spans)
}

fn parse_yaml_line(input: &str) -> Line {
    use ratatui::text::{Span, Line};
    if let Some((k, v)) = input.split_once(':') {
        let mut spans = vec![Span::styled(k, Style::default().fg(Color::Yellow)), Span::raw(":"), Span::raw(v)];
        return Line::from(spans);
    }
    Line::from(Span::raw(input))
}

fn parse_json_line(input: &str) -> Line {
    use ratatui::text::{Span, Line};
    let mut spans = Vec::new();
    let mut in_string = false;
    let mut current = String::new();
    for c in input.chars() {
        match c {
            '"' => {
                in_string = !in_string;
                current.push(c);
                if !in_string {
                    spans.push(Span::styled(current.clone(), Style::default().fg(Color::Yellow)));
                    current.clear();
                }
            }
            _ => current.push(c),
        }
    }
    if !current.is_empty() {
        spans.push(Span::raw(current));
    }
    Line::from(spans)
}

fn parse_zen_line(input: &str, syntax: ZenSyntax) -> Line {
    match syntax {
        ZenSyntax::Markdown => parse_markdown_line(input),
        ZenSyntax::Rust => parse_rust_line(input),
        ZenSyntax::Shell => parse_shell_line(input),
        ZenSyntax::Yaml => parse_yaml_line(input),
        ZenSyntax::Json => parse_json_line(input),
        ZenSyntax::None => Line::from(input),
    }
}

fn highlight_tags_line(input: &str) -> Line<'static> {
    use ratatui::text::{Span, Line};
    let mut spans = Vec::new();
    for token in input.split_whitespace() {
        if token.starts_with('#') {
            spans.push(Span::styled(token.to_string(), Style::default().fg(Color::Blue)));
        } else {
            spans.push(Span::raw(token.to_string()));
        }
        spans.push(Span::raw(" "));
    }
    Line::from(spans)
}
