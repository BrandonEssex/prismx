use ratatui::{prelude::*, widgets::{Block, Borders, Paragraph}};
use crate::state::AppState;
use crate::beamx::{render_full_border, style_for_mode};
use crate::ui::beamx::{BeamX, BeamXStyle, BeamXMode, BeamXAnimationMode};

use std::time::{SystemTime, UNIX_EPOCH};

pub fn render_zen_journal<B: Backend>(f: &mut Frame<B>, area: Rect, state: &AppState) {
    use ratatui::text::Line;
    let style = style_for_mode(&state.mode);

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
    let bg_color = match tick % 20 {
        0..=9 => Color::Rgb(18, 18, 18),
        _ => Color::Rgb(12, 12, 12),
    };
    let bg = Block::default().style(Style::default().bg(bg_color));
    f.render_widget(bg, area);

    let padding = area.width / 4;
    let usable_width = area.width - (padding * 2);

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

    // Render faded previous lines centered within the padded width
    for (i, line) in visible_lines.iter().enumerate() {
        let mut spans = line.spans.clone();
        for span in &mut spans {
            span.patch_style(Style::default().fg(Color::DarkGray));
        }
        let y = area.y + top_padding as u16 + i as u16;
        let rect = Rect::new(area.x + padding, y, usable_width, 1);
        let widget = Paragraph::new(Line::from(spans))
            .block(Block::default().borders(Borders::NONE))
            .alignment(Alignment::Center);
        f.render_widget(widget, rect);
    }

    // Animated caret for input line
    let caret = if tick % 2 == 0 { "|" } else { " " };
    let input_line = format!("> {}{}", input_text, caret);
    let input_rect = Rect::new(area.x + padding, area.y + area.height / 2, usable_width, 1);
    let input_widget = Paragraph::new(input_line)
        .style(Style::default().fg(Color::White).add_modifier(Modifier::BOLD))
        .alignment(Alignment::Center);
    f.render_widget(input_widget, input_rect);

    // Hidden side labels
    f.render_widget(
        Paragraph::new("Recent").style(Style::default().fg(Color::DarkGray)),
        Rect::new(area.x + 1, area.y + 2, padding - 2, 1),
    );
    f.render_widget(
        Paragraph::new("Aa").style(Style::default().fg(Color::DarkGray)),
        Rect::new(area.right() - padding + 1, area.y + 2, 3, 1),
    );

    if state.zen_toolbar_open {
        let mut entries = vec!["+ New".to_string(), "Open".into(), "Save".into()];
        entries.extend(state.zen_recent_files.clone());
        for (i, item) in entries.iter().enumerate() {
            let style = if i == state.zen_toolbar_index {
                Style::default().add_modifier(Modifier::REVERSED)
            } else {
                Style::default()
            };
            f.render_widget(
                Paragraph::new(item.as_str()).style(style),
                Rect::new(area.x + 1, area.y + 3 + i as u16, padding - 2, 1),
            );
        }
    }

    let unsaved_marker = if state.zen_dirty { " \u{270E}" } else { "" };
    let footer_text = format!("\u{1F4C4} {}   \u{270D} {} words{}", state.zen_current_filename, state.zen_word_count, unsaved_marker);
    let x_offset = area.width.saturating_sub(footer_text.len() as u16 + 2);
    let footer_rect = Rect::new(area.x + x_offset, area.y + area.height - 1, footer_text.len() as u16, 1);
    f.render_widget(
        Paragraph::new(footer_text).style(Style::default().fg(Color::DarkGray)),
        footer_rect,
    );

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
