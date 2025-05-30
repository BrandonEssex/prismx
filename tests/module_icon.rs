use prismx::render::module_icon::module_icon;

#[test]
fn module_icon_matches_mode() {
    assert_eq!(module_icon("gemx"), "🧠");
    assert_eq!(module_icon("zen"), "🧘");
    assert_eq!(module_icon("triage"), "🏷️");
    assert_eq!(module_icon("spotlight"), "🔍");
    assert_eq!(module_icon("settings"), "⚙️");
}
