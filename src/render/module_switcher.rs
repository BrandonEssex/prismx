use ratatui::{
    backend::Backend,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Wrap},
    Frame,
};

pub fn render_module_switcher<B: Backend>(f: &mut Frame<B>, area: Rect, index: usize) {
    let modules = [
        ("💭", "Mindmap"),
        ("🧘", "Zen"),
        ("🧭", "Triage"),
        ("⚙️", "Settings"),
    ];

    let lines: Vec<Line> = modules
        .iter()
        .enumerate()
        .map(|(i, (icon, label))| {
            let selected = i == index % modules.len();
            let prefix = if selected { "> " } else { "  " };
            let style = if selected {
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::Gray)
            };
            Line::from(vec![
                Span::styled(prefix.to_string(), style),
                Span::styled(format!("{} {}", icon, label), style),
            ])
        })
        .collect();

    let content_width = lines
        .iter()
        .map(|l| l.width() as u16)
        .max()
        .unwrap_or(0)
        .saturating_add(4);

    let width = content_width.min(area.width);
    let height = lines.len() as u16 + 2;

    let x = area.x + (area.width.saturating_sub(width)) / 2;
    let y = area.y + (area.height.saturating_sub(height)) / 2;

    let block = Block::default().title("Switch Module").borders(Borders::ALL);

    let content = Paragraph::new(lines)
        .block(block)
        .wrap(Wrap { trim: false });

    f.render_widget(content, Rect::new(x, y, width, height));
}
