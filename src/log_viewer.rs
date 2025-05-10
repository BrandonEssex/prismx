use ratatui::{
    layout::Rect,
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

pub fn render_log_viewer(f: &mut Frame<'_>, area: Rect) {
    let block = Block::default().title("Logs").borders(Borders::ALL);
    let lines = vec![
        Line::from(Span::raw("[log viewer placeholder]")),
        Line::from(Span::raw("Log file path: logs/qa_runtime.log")),
    ];
    let para = Paragraph::new(lines).block(block);
    f.render_widget(para, area);
}