use crate::state::AppState;

pub fn status_line(state: &AppState) -> String {
    match state.mode.as_str() {
        "zen" => {
            let dirty = if state.zen_dirty { " ✎" } else { "" };
            let layout = format!("{:?}", state.zen_layout_mode);
            format!("📄 {} ✍️ {} words{} [{}]", state.zen_current_filename, state.zen_word_count, dirty, layout)
        }
        "gemx" => {
            let layout = format!("{:?}", crate::gemx::layout::current_mode());
            let focus = state
                .selected
                .and_then(|id| state.nodes.get(&id))
                .map(|n| n.label.clone())
                .unwrap_or_default();
            format!("Nodes: {} Layout: {} Focus: {}", state.nodes.len(), layout, focus)
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
