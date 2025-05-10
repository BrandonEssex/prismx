// FINAL FULL FILE DELIVERY
// Filename: /src/command_bar.rs
// File Delivery Progress: 12/∞ FINAL FILES delivered

use crate::action::Action;

pub fn get_commands() -> Vec<(&'static str, Action)> {
    vec![
        ("lock node", Action::LockFocusedNode),
        ("unlock node", Action::UnlockFocusedNode),
        ("toggle sidebar", Action::ToggleSidebar),
        ("summarize node", Action::SummarizeNode),
        ("suggest links", Action::SuggestLinks),
        ("list bookmarks", Action::ListBookmarks),
        ("open comments", Action::ToggleCommentPanel),
    ]
}