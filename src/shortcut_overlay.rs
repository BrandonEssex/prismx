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
        "q - Quit",
        "e - Edit Node",
        "m - Toggle Layout",
        "c - Context Menu",
        "i - Open Inbox",
        "? - Show Help",
        "z - Zen Mode",
    ];

    let lines: Vec<Line> = bindings
        .iter()
        .map(|b| Line::from(Span::styled(*b, Style::default().fg(Color::Cyan))))
        .collect();

    let block = Block::default().title("Shortcuts").borders(Borders::ALL);
    let para = Paragraph::new(lines).block(block);
    let width = 30;
    let height = bindings.len() as u16 + 2;

    frame.render_widget(para, Rect::new(area.width - width - 1, area.y + 1, width, height));
}