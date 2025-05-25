use ratatui::{prelude::*, widgets::{Block, Borders, Paragraph, Wrap}};
use crate::state::AppState;
use crate::beamx::{render_full_border, style_for_mode};
use crate::ui::beamx::{BeamX, BeamXStyle, BeamXMode, BeamXAnimationMode};

use std::time::{SystemTime, UNIX_EPOCH};

pub fn render_zen_journal<B: Backend>(f: &mut Frame<B>, area: Rect, state: &AppState) {
    use ratatui::text::{Span, Line};
    let style = style_for_mode(&state.mode);

    let total_height = area.height as usize;
    let total_width = area.width as usize;

    if total_height < 4 || total_width < 10 {
        return;
    }

    // Frame tick for simple animations
    let tick = (SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis()
        / 300) as u64;

    // Background pulse effect
    let bg_color = match tick % 20 {
        0..=9 => Color::Rgb(18, 18, 18),
        _ => Color::Rgb(12, 12, 12),
    };
    let bg = Block::default().style(Style::default().bg(bg_color));
    f.render_widget(bg, area);

    let zen_snapshot: Vec<String> = state.zen_buffer.clone();
    let input_text = zen_snapshot.last().cloned().unwrap_or_default();

    // Build lines without current input
    let raw_lines: Vec<Line> = zen_snapshot
        .iter()
        .take(zen_snapshot.len().saturating_sub(1))
        .map(|s| parse_markdown_line(s))
        .collect();
    let top_padding = 3;
    let usable_height = total_height.saturating_sub(top_padding);
    let start_line = raw_lines.len().saturating_sub(usable_height);
    let visible_lines = &raw_lines[start_line..];
    let h_margin = (total_width as f32 * 0.20) as usize;

    let mut padded_lines = std::iter::repeat(Line::from(""))
        .take(top_padding)
        .collect::<Vec<_>>();
    for (i, line) in visible_lines.iter().enumerate() {
        let style = if visible_lines.len().saturating_sub(1) - i == 0 {
            Style::default().fg(Color::White)
        } else {
            Style::default().fg(Color::DarkGray)
        };
        let mut padded = vec![Span::raw(" ".repeat(h_margin))];
        let mut spans = line.spans.clone();
        for span in &mut spans {
            span.patch_style(style);
        }
        padded.extend(spans);
        padded_lines.push(Line::from(padded));
    }

    let widget = Paragraph::new(padded_lines)
        .block(Block::default().title("Zen").borders(Borders::NONE))
        .wrap(Wrap { trim: false });

    f.render_widget(widget, area);

    // Animated caret for input line
    let caret = if tick % 2 == 0 { "|" } else { " " };
    let input_line = format!("> {}{}", input_text, caret);
    let x_offset = (area.width.saturating_sub(input_line.len() as u16)) / 2;
    let centered = Rect::new(area.x + x_offset, area.y + area.height / 2, area.width, 1);
    let input_widget = Paragraph::new(input_line).style(Style::default().fg(Color::White));
    f.render_widget(input_widget, centered);

    render_full_border(f, area, &style, true, false);
    let beamx = BeamX {
        tick,
        enabled: true,
        mode: BeamXMode::Zen,
        style: BeamXStyle::from(BeamXMode::Zen),
        animation: BeamXAnimationMode::PulseEntryRadiate,
    };
    beamx.render(f, area);
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
