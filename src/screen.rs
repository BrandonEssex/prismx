use ratatui::Terminal;
use ratatui::backend::Backend;
use crate::state::AppState;

pub struct Screen {
    state: AppState,
}

impl Screen {
    pub fn new() -> Self {
        Self {
            state: AppState::default(),
        }
    }

    pub fn run<B: Backend>(&mut self, terminal: &mut Terminal<B>) -> Result<(), Box<dyn std::error::Error>> {
        loop {
            terminal.draw(|f| {
                // Placeholder rendering logic
                let size = f.size();
                f.render_widget(
                    ratatui::widgets::Paragraph::new("PrismX Running..."),
                    size,
                );
            })?;

            // Exit simulation
            break;
        }
        Ok(())
    }
}