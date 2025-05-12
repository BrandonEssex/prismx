use ratatui::{
    backend::Backend,
    layout::Rect,
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

pub struct AlertBanner {
    pub message: String,
    pub visible: bool,
}

impl AlertBanner {
    pub fn new() -> Self {
        Self {
            message: String::new(),
            visible: false,
        }
    }

    pub fn show(&mut self, msg: &str) {
        self.message = msg.to_string();
        self.visible = true;
    }

    pub fn hide(&mut self) {
        self.visible = false;
    }

    pub fn render<B: Backend>(&self, f: &mut Frame<B>, area: Rect) {
        if !self.visible {
            return;
        }

        let banner = Paragraph::new(self.message.clone())
            .block(
                Block::default()
                    .title("⚠ Alert")
                    .borders(Borders::ALL)
                    .border_style(
                        Style::default()
                            .fg(Color::Red)
                            .add_modifier(Modifier::BOLD),
                    ),
            )
            .style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD));

        f.render_widget(banner, area);
    }
}
