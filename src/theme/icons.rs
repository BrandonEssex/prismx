// Node glyphs used throughout the UI
pub const ROOT_NODE: &str = "\u{1F9E0}"; // 🧠
pub const CHILD_NODE: &str = "└▶";
pub const SIBLING_NODE: &str = "↔";

// ───── Module Icons ─────
pub const ICON_GEMX: &str = "🧠";
pub const ICON_ZEN: &str = "🧘";
pub const ICON_TRIAGE: &str = "🏷️";
pub const ICON_SETTINGS: &str = "⚙️";
pub const ICON_SPOTLIGHT: &str = "🔍";

// Simple ASCII fallbacks when Nerd Font icons aren't available
pub const FALLBACK_ICON_GEMX: &str = "[G]";
pub const FALLBACK_ICON_ZEN: &str = "[Z]";
pub const FALLBACK_ICON_TRIAGE: &str = "[T]";
pub const FALLBACK_ICON_SETTINGS: &str = "[S]";
pub const FALLBACK_ICON_SPOTLIGHT: &str = "[?]";

// ───── Status Icons ─────
pub const ICON_SYNC: &str = "";    // nf-fa-wifi
pub const ICON_SHELL: &str = "";   // nf-fa-terminal
pub const ICON_SUCCESS: &str = ""; // nf-fa-check
pub const ICON_LOG: &str = "";     // nf-fa-rss
pub const ICON_CLOCK: &str = "";   // nf-fa-history

// ───── Overlay Icons ─────
pub const ICON_NODE: &str = "";    // nf-fa-sticky_note_o
pub const ICON_TERMINAL: &str = ""; // nf-oct-terminal
pub const ICON_DATE: &str = "";    // nf-oct-calendar
pub const ICON_LINK: &str = "";    // nf-fa-link

pub const FALLBACK_ICON_NODE: &str = "[N]";
pub const FALLBACK_ICON_TERMINAL: &str = "[T]";
pub const FALLBACK_ICON_DATE: &str = "[D]";
pub const FALLBACK_ICON_LINK: &str = "[L]";

/// Determine if Nerd Font icons should be used.
pub fn nerd_font_enabled() -> bool {
    std::env::var("PRISMX_NO_NERD_FONT").is_err()
}

pub fn terminal_icon() -> &'static str {
    if nerd_font_enabled() { ICON_TERMINAL } else { FALLBACK_ICON_TERMINAL }
}

