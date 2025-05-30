use prismx::spotlight::command_suggestions;

#[test]
fn autocomplete_matches_partial_command() {
    let results = command_suggestions("t");
    assert!(results.contains(&"triage"));
}

#[test]
fn autocomplete_updates_dynamically() {
    let initial = command_suggestions("");
    assert!(initial.contains(&"triage"));
    let updated = command_suggestions("s");
    assert_eq!(updated, vec!["settings"]);
}

#[test]
fn autocomplete_no_matches_returns_empty() {
    let results = command_suggestions("unknown");
    assert!(results.is_empty());
}

#[test]
fn autocomplete_slash_and_no_slash_equivalent() {
    let no_slash = command_suggestions("zen");
    let with_slash = command_suggestions("/zen");
    assert_eq!(no_slash, with_slash);
}
