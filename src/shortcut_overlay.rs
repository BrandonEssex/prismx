use ratatui::{
    layout::Rect,
    widgets::{Block, Borders, Paragraph},
    text::{Line, Span},
    style::{Style, Color},
    Frame,
};

pub fn render_shortcuts<B>(frame: &mut Frame<'_>, area: Rect, visible: bool)
where
    B: ratatui::backend::Backend,
{
    if !visible {
        return;
    }

    let bindings = vec![
        "Ctrl+Q - Quit",
        "Ctrl+E - Edit Node",
        "Ctrl+N - New Sibling Node",
        "Ctrl+Tab - New Child Node",
        "Ctrl+D - Duplicate Node",
        "Ctrl+Backspace - Delete Node",
        "Ctrl+M - Toggle Layout",
        "Ctrl+Z - Zen Mode",
        "Ctrl+/ - Show Shortcuts",
        "Ctrl+L - View Logs",
    ];

    let lines: Vec<Line> = bindings
        .iter()
        .map(|b| Line::from(Span::styled(*b, Style::default().fg(Color::Cyan))))
        .collect();

    let block = Block::default().title("Shortcuts").borders(Borders::ALL);
    let para = Paragraph::new(lines).block(block);
    let width = 38;
    let height = bindings.len() as u16 + 2;

    frame.render_widget(para, Rect::new(area.width - width - 1, area.y + 1, width, height));
}