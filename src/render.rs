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
    use ratatui::widgets::{Wrap};
    use ratatui::text::{Span, Line};
    use ratatui::layout::Alignment;

    let lines: Vec<Line> = state.zen_buffer.iter().map(|line| parse_markdown_line(line)).collect();

    let widget = Paragraph::new(lines)
        .block(Block::default().title("Zen").borders(Borders::ALL))
        .alignment(Alignment::Left)
        .style(Style::default().fg(Color::Green))
        .wrap(Wrap { trim: false });

    let scroll_offset = state.zen_buffer.len().saturating_sub((area.height as usize).saturating_sub(5));
    f.render_widget(widget.scroll((scroll_offset as u16, 0)), area);
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


pub fn render_mindmap<B: Backend>(f: &mut Frame<B>, area: Rect, state: &AppState) {
    let layout = Block::default()
        .borders(Borders::ALL)
        .title(if state.edit_mode { "Mindmap (Edit)" } else { "Mindmap" });
    f.render_widget(layout, area);

    let offset_y = area.y + 1;

    for (i, (depth, node)) in state.flat_nodes.iter().enumerate() {
        let y = offset_y + i as u16;
        if y >= area.bottom() {
            break;
        }

        let n = node.borrow();
        let label = &n.label;

        let prefix = if !n.children.is_empty() {
            if n.collapsed { "[+]" } else { "[-]" }
        } else {
            "   "
        };

        let indent = "  ".repeat(*depth);
        let content = if i == state.active_node {
            format!("> {}{} {}", indent, prefix, label)
        } else {
            format!("  {}{} {}", indent, prefix, label)
        };

        let style = if i == state.active_node {
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD | Modifier::UNDERLINED)
        } else {
            Style::default().fg(Color::White)
        };

        let para = Paragraph::new(content).style(style);
        f.render_widget(para, Rect::new(area.x + 2, y, area.width - 4, 1));
    }
}

pub fn render_keymap_overlay<B: Backend>(f: &mut Frame<B>, area: Rect) {
    let block = Block::default().title("Keymap").borders(Borders::ALL);
    f.render_widget(block, area);

    let content = Paragraph::new(
        "\
        Ctrl+Q = Quit\n\
        Ctrl+M = Mindmap\n\
        Ctrl+Z = Zen\n\
        Ctrl+E = Edit Mode\n\
        Ctrl+T = Triage\n\
        Ctrl+H = Help\n\
        Ctrl+. = Settings\n\
        Alt+Space = Spotlight\n\
        Shift+Tab = Switch Module\n\
        Tab = Add Child\n\
        Enter = Add Sibling\n\
        Shift+Backspace = Delete\n\
        Esc = Exit overlay/edit\n\
        ",
    );

    f.render_widget(content, Rect::new(area.x + 1, area.y + 1, area.width - 2, area.height - 2));
}

pub fn render_triage<B: Backend>(f: &mut Frame<B>, area: Rect) {
    let block = Block::default()
        .title("Triage Panel")
        .borders(Borders::ALL)
        .style(Style::default().fg(Color::Red));

    let content = Paragraph::new(
        "• Mindmap rendering: OK\n\
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
