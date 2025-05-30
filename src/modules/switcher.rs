use crate::theme::layout::OVERLAY_WIDTH;

pub const SWITCHER_WIDTH: u16 = OVERLAY_WIDTH;

pub const MODULES: [(&str, &str); 5] = [
    ("\u{1F4AD}", "Mindmap"),    // 💭
    ("\u{1F9D8}", "Zen"),        // 🧘
    ("\u{1F3E5}", "Triage"),     // 🏥
    ("\u{2699}\u{FE0F}", "Settings"), // ⚙️
    ("\u{1F50C}", "Plugins"),    // 🔌
];

pub fn mode_for_index(index: usize) -> &'static str {
    match index % MODULES.len() {
        0 => "gemx",
        1 => "zen",
        2 => "triage",
        3 => "settings",
        4 => "plugin",
        _ => "gemx",
    }
}

pub fn index_for_mode(mode: &str) -> usize {
    match mode {
        "gemx" => 0,
        "zen" => 1,
        "triage" => 2,
        "settings" => 3,
        "plugin" => 4,
        _ => 0,
    }
}
