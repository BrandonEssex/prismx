use ratatui::text::{Line, Span};
use ratatui::style::{Color, Style};

/// Extract all hashtags from a line of text (e.g., "#tag")
pub fn extract_tags(text: &str) -> Vec<String> {
    let mut tags = Vec::new();
    let mut chars = text.chars().peekable();
    while let Some(c) = chars.next() {
        if c == '#' {
            let mut tag = String::new();
            while let Some(&ch) = chars.peek() {
                if ch.is_alphanumeric() || ch == '_' || ch == '-' {
                    tag.push(ch);
                    chars.next();
                } else {
                    break;
                }
            }
            if !tag.is_empty() {
                tags.push(format!("#{}", tag));
            }
        }
    }
    tags
}

pub fn highlight_tags_line(input: &str) -> Line<'static> {
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
