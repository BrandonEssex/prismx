use ratatui::{backend::Backend, style::Style, widgets::Paragraph, Frame, text::{Span, Spans}};
use crate::ui::layout::Rect;
use crate::state::AppState;
use crate::ui::borders::draw_rounded_border;
use crate::render::module_icon::{module_icon, module_label};
use crate::theme::icons;
use crate::ui::shortcuts::shortcuts_for;
use crate::modules::triage::render::{completion_streak, done_sparkline, progress_bar};
use crate::ui::dock::render_dock;
use crate::layout::RESERVED_ZONE_W;
use unicode_width::UnicodeWidthStr;

/// Utility to generate a default status string for the current mode.
pub fn status_line(state: &AppState) -> String {
    match state.mode.as_str() {
        "zen" => {
            let dirty = if state.zen_dirty { format!(" {}", icons::IC_EDIT) } else { String::new() };
            let layout = format!("{:?}", state.zen_layout_mode);
            format!(
                "{} {} {} {} words{} [{}]",
                icons::IC_DOC,
                state.zen_current_filename,
                icons::IC_PEN,
                state.zen_word_count,
                dirty,
                layout
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
            let bar = progress_bar(now, triton, done);
            let streak = completion_streak(&state.triage_entries);
            let spark = done_sparkline(&state.triage_entries, 7);
            let current = state
                .triage_entries
                .get(state.triage_focus_index)
                .filter(|e| !e.archived)
                .map(|e| e.text.clone())
                .unwrap_or_default();
            format!(
                "#NOW:{} #TRITON:{} #DONE:{} {} {}{} {} | {}",
                now,
                triton,
                done,
                bar,
                icons::IC_FIRE,
                streak,
                spark,
                current
            )
        }
        _ => format!("Mode: {}", state.mode),
    }
}

/// Render the contextual status bar with optional tips.
pub fn render_status<B: Backend>(f: &mut Frame<B>, area: Rect, state: &mut AppState) {
    let beam = state.beam_style_for_mode(&state.mode);
    let border_style = Style::default().fg(beam.border_color);
    draw_rounded_border(f, area, border_style);

    let mut message = if !state.status_message.is_empty() {
        state.status_message.clone()
    } else if state.show_keymap {
        let module_name = state.mode.to_string();
        let shortcuts = crate::ui::shortcuts::shortcuts_for(&module_name);
        shortcuts
            .iter()
            .take(3)
            .map(|(k, a)| format!("{k}: {a}"))
            .collect::<Vec<_>>()
            .join(" | ")
    } else {
        status_line(state)
    };

    // Prefix active module icon for quick visual context
    let show_heart = matches!(state.mode.as_str(), "zen" | "gemx")
        && state.heartbeat_mode != crate::state::HeartbeatMode::Silent;
    let prefix = module_icon(&state.mode).to_string();

    let content_style = Style::default().fg(beam.status_color);
    let width = area.width.saturating_sub(2);

    let dock_width = state.favorite_entries().len() as u16 * 3;
    let heart_w = if show_heart { 3 } else { 0 };
    let zoom_text = format!("Zoom: {:.1}x", state.zoom_scale);
    let zoom_w = zoom_text.len() as u16 + 2;
    let icon_content = format!("{} {}", module_icon(&state.mode), module_label(&state.mode));
    let icon_w = UnicodeWidthStr::width(icon_content.as_str()) as u16 + 2;
    let offset = RESERVED_ZONE_W as u16 + icon_w + zoom_w + 1;
    let available = width.saturating_sub(dock_width + heart_w + offset);

    let prefix_len = prefix.len() + 1; // include trailing space
    let mut msg_display = message;
    if msg_display.len() as u16 > available.saturating_sub(prefix_len as u16) {
        msg_display.truncate(available.saturating_sub(prefix_len as u16) as usize);
    }

    let spans = Spans::from(vec![
        Span::styled(module_icon(&state.mode).to_string(), Style::default().fg(beam.border_color)),
        Span::raw(" "),
        Span::raw(msg_display),
    ]);

    f.render_widget(
        Paragraph::new(spans).style(content_style),
        Rect::new(area.x + 1, area.y + 1, available, 1),
    );

    // draw dock icons aligned to the right inside the same status bar
    render_dock(f, area, state);
}
