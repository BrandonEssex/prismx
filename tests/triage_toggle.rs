use prismx::state::AppState;
use prismx::triage::state::{TriageEntry, TriageSource};

#[test]
fn triage_toggle_tag_adds_and_removes() {
    let mut state = AppState::default();
    state.triage_entries.push(TriageEntry::new(0, "fix bug", TriageSource::Zen));
    state.triage_focus_index = 0;

    state.triage_toggle_tag("#now");
    assert!(state.triage_entries[0].tags.contains(&"#now".to_string()));
    assert!(state.triage_entries[0].text.contains("#now"));

    state.triage_toggle_tag("#now");
    assert!(!state.triage_entries[0].tags.contains(&"#now".to_string()));
    assert!(!state.triage_entries[0].text.contains("#now"));
}
