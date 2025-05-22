use ratatui::{
    backend::Backend,
    layout::{Alignment, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Paragraph},
    prelude::Line,
    Frame,
};

use crate::state::AppState;


pub fn render_status_bar<B: Backend>(f: &mut Frame<B>, area: Rect, status: &str) {
    let block = Block::default()
        .borders(Borders::ALL)
        .title("Status")
        .style(Style::default().bg(Color::Black).fg(Color::White));
    let content = Paragraph::new(status);
    f.render_widget(block, area);
    f.render_widget(content, Rect::new(area.x + 1, area.y + 1, area.width - 2, 1));
}

pub fn render_zen_journal<B: Backend>(f: &mut Frame<B>, area: Rect, state: &AppState) {
    use ratatui::widgets::Wrap;
    use ratatui::text::{Span, Line};
    use ratatui::layout::Alignment;

    let total_height = area.height as usize;
    let total_width = area.width as usize;

    if total_height < 4 || total_width < 10 {
        return;
    }

    let zen_snapshot: Vec<String> = state.zen_buffer.clone();

    let raw_lines: Vec<Line> = if zen_snapshot.is_empty() {
        vec![Line::from(" ")]
    } else {
        zen_snapshot.iter().map(|line| parse_markdown_line(line)).collect()
    };

    let top_padding = 3;
    let usable_height = total_height.saturating_sub(top_padding);
    let start_line = raw_lines.len().saturating_sub(usable_height);
    let visible_lines = &raw_lines[start_line..];

    // ⬅️➡️ Horizontal text pane: 20% margin on each side
    let h_margin = (total_width as f32 * 0.20) as usize;

    let mut padded_lines: Vec<Line> = std::iter::repeat(Line::from("")).take(top_padding).collect();

    for line in visible_lines {
        let mut padded = vec![Span::raw(" ".repeat(h_margin))];
        padded.extend(line.spans.clone());
        padded_lines.push(Line::from(padded));
    }

    let widget = Paragraph::new(padded_lines)
        .block(Block::default().title("Zen").borders(Borders::ALL))
        .alignment(Alignment::Left)
        .style(Style::default().fg(Color::Green))
        .wrap(Wrap { trim: false });

    f.render_widget(widget, area);
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
            chars.next(); // consume second '*'
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

pub fn render_keymap_overlay<B: Backend>(f: &mut Frame<B>, area: Rect) {
    use ratatui::widgets::{Block, Borders, Paragraph};
    use ratatui::text::Line;

    let keys = vec![
        "Ctrl+C  = Quit",
        "Ctrl+Z  = Undo",
        "Ctrl+Shift+Z = Redo",
        "Ctrl+R  = Start/Finish Drag",
        "Ctrl+L  = Start/Finish Link",
        "Ctrl+N  = Free Node",
        "Tab     = Add Child",
        "Enter   = Add Sibling",
        "Ctrl+D  = Delete Node",
        "Ctrl+W  = Drill Down",
        "Ctrl+Q  = Pop Up",
        "Ctrl+T  = Toggle Collapse",
        "Ctrl+X  = Save Zen",
        "Ctrl+Space = Module Switcher",
        "Ctrl+Y  = Triage",
        "Ctrl+H  = Help",
        "Ctrl+.  = Settings",
        "Alt+Space = Spotlight",
        "Esc     = Close Overlay / Exit Mode"
    ];

    let content = Paragraph::new(keys.join("\n"))
        .block(Block::default().title("Keymap").borders(Borders::ALL));

    f.render_widget(content, Rect::new(area.x + 1, area.y + 1, area.width - 2, area.height - 2));
}

pub fn render_triage<B: Backend>(f: &mut Frame<B>, area: Rect) {
    let block = Block::default()
        .title("Triage Panel")
        .borders(Borders::ALL)
        .style(Style::default().fg(Color::Red));

    let content = Paragraph::new(
        "• GemX rendering: OK\n\
         • Node editing: OK\n\
         • Zen scroll: OK\n\
         • Triage display: Working"
    );

    f.render_widget(block, area);
    f.render_widget(content, Rect::new(area.x + 2, area.y + 1, area.width - 4, area.height - 2));
}

pub fn render_spotlight<B: Backend>(f: &mut Frame<B>, area: Rect, input: &str) {
    let width = area.width.min(60);
    let x_offset = area.x + (area.width.saturating_sub(width)) / 2;
    let y_offset = area.y + area.height / 3;

    let spotlight_area = Rect::new(x_offset, y_offset, width, 3);

    let block = Block::default()
        .title("Spotlight")
        .borders(Borders::ALL)
        .style(Style::default().fg(Color::Cyan));

    let paragraph = Paragraph::new(format!("> {}", input))
        .block(block)
        .style(Style::default().fg(Color::White))
        .alignment(Alignment::Left);

    f.render_widget(paragraph, spotlight_area);
}

pub fn render_module_switcher<B: Backend>(f: &mut Frame<B>, area: Rect, index: usize) {
    use ratatui::widgets::Wrap;

    let modules = ["Mindmap", "Zen", "Settings", "Triage"];
    let selected = modules[index % modules.len()];
    let width = 30;
    let height = 5;

    let x = area.x + (area.width.saturating_sub(width)) / 2;
    let y = area.y + (area.height.saturating_sub(height)) / 2;

    let block = Block::default()
        .title("Switch Module")
        .borders(Borders::ALL)
        .style(Style::default().fg(Color::Magenta));

    let content = Paragraph::new(format!("[ {} ]", selected))
        .block(block)
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: false });

    f.render_widget(content, Rect::new(x, y, width, height));
}
