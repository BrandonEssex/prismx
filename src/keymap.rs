use std::collections::HashMap;

#[derive(Debug)]
pub enum Action {
    Quit,
    ToggleZen,
    OpenSettings,
    OpenSpotlight,
    CreateNode,
    MoveFocusUp,
    MoveFocusDown,
    MoveFocusLeft,
    MoveFocusRight,
    DeleteNode,
    EditNode,
    SaveSnapshot,
}

pub fn default_keymap() -> HashMap<&'static str, Action> {
    use Action::*;
    HashMap::from([
        ("C-q", Quit),
        ("C-z", ToggleZen),
        ("C-.", OpenSettings),
        ("M-Space", OpenSpotlight),
        ("Enter", CreateNode),
        ("C-n", CreateNode),
        ("C-k", MoveFocusUp),
        ("C-j", MoveFocusDown),
        ("C-h", MoveFocusLeft),
        ("C-l", MoveFocusRight),
        ("C-x", DeleteNode),
        ("C-w", EditNode),
        ("C-s", SaveSnapshot),
    ])
}

pub fn show_overlay() {
    let keymap = default_keymap();

    println!("┌─────────────────────┐");
    println!("│  KEYMAP SHORTCUTS   │");
    println!("├─────────────────────┤");
    for (key, action) in keymap {
        println!("│ {:<12} → {:<12} │", key, format!("{:?}", action));
    }
    println!("└─────────────────────┘");
}
