use crate::action::Action;

pub fn get_command_action_map() -> Vec<(&'static str, Action)> {
    vec![
        ("quit", Action::Quit),
        ("toggle zen", Action::ToggleZenMode),
        ("edit node", Action::EnterEditNode),
        ("expand node", Action::ExpandNode),
        ("collapse node", Action::CollapseNode),
        ("lock node", Action::LockFocusedNode),
        ("unlock node", Action::UnlockFocusedNode),
        ("toggle panel", Action::TogglePrismPanel),
    ]
}