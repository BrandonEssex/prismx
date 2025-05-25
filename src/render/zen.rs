use ratatui::{prelude::*, widgets::{Block, Borders, Paragraph}};
use crate::state::{AppState, ZenSyntax, ZenTheme, ZenJournalView};
use crate::beamx::{render_full_border, style_for_mode};
use crate::ui::beamx::{BeamX, BeamXStyle, BeamXMode, BeamXAnimationMode};

use std::time::{SystemTime, UNIX_EPOCH};

pub fn render_zen_journal<B: Backend>(f: &mut Frame<B>, area: Rect, state: &AppState) {
    use ratatui::text::{Line, Span};
    let mut style = style_for_mode(&state.mode);
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
}

fn render_compose<B: Backend>(f: &mut Frame<B>, area: Rect, state: &AppState, tick: u64) {
    use ratatui::text::Span;
    let padding = area.width / 4;
    let usable_width = area.width - padding * 2;
    let caret = if tick % 2 == 0 { "|" } else { " " };
    let timestamp = chrono::Local::now().format("%H:%M").to_string();
    let input = format!("{} {}{}", timestamp, state.zen_compose_input, caret);
    let input_rect = Rect::new(area.x + padding, area.bottom().saturating_sub(2), usable_width, 1);
    let widget = Paragraph::new(input).block(Block::default().borders(Borders::NONE));
    f.render_widget(widget, input_rect);
    render_full_border(f, area, &style_for_mode(&state.mode), true, false);
}

fn render_review<B: Backend>(f: &mut Frame<B>, area: Rect, state: &AppState) {
    let padding = area.width / 4;
    let usable_width = area.width - padding * 2;
    let mut y = area.y + 1;
    for entry in state.zen_journal_entries.iter().rev() {
        let text = format!("\u{1F551} {}\n{}", entry.timestamp.format("%I:%M %p"), entry.text);
        let rect = Rect::new(area.x + padding, y, usable_width, 3);
        let p = Paragraph::new(text).block(Block::default().borders(Borders::BOTTOM));
        f.render_widget(p, rect);
        y = y.saturating_add(3);
        if y > area.bottom() { break; }
    }
    render_full_border(f, area, &style_for_mode(&state.mode), true, false);
}

fn parse_markdown_line(input: &str) -> Line {
    use ratatui::text::{Span, Line};
    use ratatui::style::Modifier;

    if input.starts_with("### ") {
        return Line::from(Span::styled(&input[4..], Style::default().add_modifier(Modifier::ITALIC)));
    } else if input.starts_with("## ") {
        return Line::from(Span::styled(&input[3..], Style::default().add_modifier(Modifier::BOLD)));
    } else if input.starts_with("# ") {
        return Line::from(Span::styled(&input[2..], Style::default().add_modifier(Modifier::BOLD | Modifier::UNDERLINED)));
    } else if input.starts_with("- ") || input.starts_with("* ") {
        return Line::from(vec![
            Span::styled("• ", Style::default().add_modifier(Modifier::BOLD)),
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
            spans.push(Span::styled(bold, Style::default().add_modifier(Modifier::BOLD)));
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
