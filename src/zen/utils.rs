use ratatui::text::{Line, Span};
use ratatui::style::{Color, Style, Modifier};

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

/// Extract tags and normalize to lowercase for classification
pub fn parse_tags(text: &str) -> Vec<String> {
    extract_tags(text)
        .into_iter()
        .map(|t| t.to_lowercase())
        .collect()
}

pub fn highlight_tags_line(input: &str) -> Line<'static> {
    let mut spans = Vec::new();
    for token in input.split_whitespace() {
        let span = if token.starts_with("**") && token.ends_with("**") && token.len() > 4 {
            Span::styled(
                token[2..token.len() - 2].to_string(),
                Style::default().add_modifier(Modifier::BOLD),
            )
        } else if token.starts_with('*') && token.ends_with('*') && token.len() > 2 {
            Span::styled(
                token[1..token.len() - 1].to_string(),
                Style::default().add_modifier(Modifier::DIM),
            )
        } else if token.starts_with('`') && token.ends_with('`') && token.len() > 2 {
            Span::styled(
                token[1..token.len() - 1].to_string(),
                Style::default().fg(Color::Yellow),
            )
        } else if token.starts_with('#') {
            Span::styled(token.to_string(), Style::default().fg(Color::Blue))
        } else {
            Span::raw(token.to_string())
        };
        spans.push(span);
        spans.push(Span::raw(" "));
    }
    Line::from(spans)
}
