use ratatui::{backend::Backend, style::Style, widgets::Paragraph, Frame};
use crate::ui::layout::Rect;
use crate::state::AppState;
use crate::ui::borders::draw_rounded_border;
use crate::render::module_icon::{module_icon, module_label};
use crate::ui::shortcuts::SHORTCUTS;

/// Utility to generate a default status string for the current mode.
pub fn status_line(state: &AppState) -> String {
    match state.mode.as_str() {
        "zen" => {
            let dirty = if state.zen_dirty { " ✎" } else { "" };
            let layout = format!("{:?}", state.zen_layout_mode);
            format!(
                "📄 {} ✍️ {} words{} [{}]",
                state.zen_current_filename, state.zen_word_count, dirty, layout
            )
        }
        "gemx" => {
            let layout = format!("{:?}", crate::gemx::layout::current_mode());
            let focus = state
                .selected
                .and_then(|id| state.nodes.get(&id))
                .map(|n| n.label.clone())
                .unwrap_or_default();
            format!(
                "Nodes: {} Layout: {} Focus: {}",
                state.nodes.len(),
                layout,
                focus
            )
        }
        "triage" => {
            let (now, triton, done) = crate::triage::state::tag_counts(state);
            let current = state
                .triage_entries
                .iter()
                .find(|e| !e.archived)
                .map(|e| e.text.clone())
                .unwrap_or_default();
            format!("#NOW:{} #TRITON:{} #DONE:{} | {}", now, triton, done, current)
        }
        _ => format!("Mode: {}", state.mode),
    }
}

/// Render the contextual status bar with optional tips.
pub fn render_status<B: Backend>(f: &mut Frame<B>, area: Rect, state: &AppState) {
    let beam = state.beam_style_for_mode(&state.mode);
    let border_style = Style::default().fg(beam.border_color);
    draw_rounded_border(f, area, border_style);

    let mut text = if !state.status_message.is_empty() {
        state.status_message.clone()
    } else if state.show_keymap {
        SHORTCUTS
            .iter()
            .take(3)
            .map(|(k, a)| format!("{k}: {a}"))
            .collect::<Vec<_>>()
            .join(" | ")
    } else {
        status_line(state)
    };

    let content_style = Style::default().fg(beam.status_color);
    let width = area.width.saturating_sub(2);
    if text.len() as u16 > width {
        text.truncate(width as usize);
    }
    f.render_widget(
        Paragraph::new(text).style(content_style),
        Rect::new(area.x + 1, area.y + 1, width, 1),
    );
}
