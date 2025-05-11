use ratatui::{
    layout::Rect,
    widgets::{Block, Borders, Paragraph},
    text::Text,
    Frame,
};

pub fn render_command_bar(f: &mut Frame<'_>, area: Rect, input: &str) {
    let block = Paragraph::new(Text::from(format!("> {}", input)))
        .block(Block::default().title("Launcher").borders(Borders::ALL));

    f.render_widget(block, area);
}