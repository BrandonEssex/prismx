use ratatui::{prelude::*, widgets::{Block, Borders, Paragraph, Wrap}};
use crate::state::AppState;
use crate::beamx::{render_beam_logo, render_full_border, style_for_mode};

pub fn render_zen_journal<B: Backend>(f: &mut Frame<B>, area: Rect, state: &AppState) {
    use ratatui::text::{Span, Line};
    let style = style_for_mode(&state.mode);

    let total_height = area.height as usize;
    let total_width = area.width as usize;

    if total_height < 4 || total_width < 10 {
        return;
    }

    let zen_snapshot: Vec<String> = state.zen_buffer.clone();
    let raw_lines: Vec<Line> = zen_snapshot.iter().map(|s| parse_markdown_line(s)).collect();
    let top_padding = 3;
    let usable_height = total_height.saturating_sub(top_padding);
    let start_line = raw_lines.len().saturating_sub(usable_height);
    let visible_lines = &raw_lines[start_line..];
    let h_margin = (total_width as f32 * 0.20) as usize;

    let mut padded_lines = std::iter::repeat(Line::from("")).take(top_padding).collect::<Vec<_>>();
    for line in visible_lines {
        let mut padded = vec![Span::raw(" ".repeat(h_margin))];
        padded.extend(line.spans.clone());
        padded_lines.push(Line::from(padded));
    }

    let widget = Paragraph::new(padded_lines)
        .block(Block::default().title("Zen").borders(Borders::NONE))
        .style(Style::default().fg(Color::Green))
        .wrap(Wrap { trim: false });

    f.render_widget(widget, area);
    render_full_border(f, area, &style);
    render_beam_logo(f, area, &style);
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
