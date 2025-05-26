use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders};

use crate::ui::components::feed::render_feed;
use crate::state::AppState;
use super::logic;
use crate::beamx::render_full_border;

pub fn render_panel<B: Backend>(f: &mut Frame<B>, area: Rect, state: &AppState) {
    let style = state.beam_style_for_mode("triage");
    let entries = logic::collect(&state.zen_journal_entries);
    let block = Block::default().title("Triage").borders(Borders::NONE);
    f.render_widget(block, area);
    let inner = Rect::new(area.x + 1, area.y + 1, area.width - 2, area.height - 2);
    render_feed(f, inner, &entries);
    render_full_border(f, area, &style, true, false);
}
