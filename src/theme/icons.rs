// Node glyphs used throughout the UI
pub const ROOT_NODE: &str = "\u{1F9E0}"; // 🧠
pub const CHILD_NODE: &str = "└▶";
pub const SIBLING_NODE: &str = "↔";

// ───── Symbolic Icons ─────
pub const IC_MINDMAP: &str = "🧠";
pub const IC_NOTE: &str = "💭";
pub const IC_ZEN: &str = "🧘";
pub const IC_FIRE: &str = "🔥";
pub const IC_DOC: &str = "📄";
pub const IC_PEN: &str = "✍️";
pub const IC_EDIT: &str = "✎";

// ───── Module Icons ─────
pub const IC_GEMX: &str = IC_MINDMAP;
pub const IC_ZEN_MODE: &str = IC_ZEN;
pub const IC_TAG: &str = "🏷️";
pub const IC_SETTINGS: &str = "⚙️";
pub const IC_SPOTLIGHT: &str = "🔍";
pub const IC_PLUGIN: &str = "🔌";

// Simple ASCII fallbacks when Nerd Font icons aren't available
pub const FALLBACK_IC_GEMX: &str = "[G]";
pub const FALLBACK_IC_ZEN_MODE: &str = "[Z]";
pub const FALLBACK_IC_TAG: &str = "[T]";
pub const FALLBACK_IC_SETTINGS: &str = "[S]";
pub const FALLBACK_IC_SPOTLIGHT: &str = "[?]";
pub const FALLBACK_IC_PLUGIN: &str = "[P]";

// ───── Status Icons ─────
pub const IC_SYNC: &str = "";    // nf-fa-wifi
pub const IC_SHELL: &str = "";   // nf-fa-terminal
pub const IC_SUCCESS: &str = ""; // nf-fa-check
pub const IC_LOG: &str = "";     // nf-fa-rss
pub const IC_CLOCK: &str = "";   // nf-fa-history

// ───── Overlay Icons ─────
pub const IC_NODE: &str = "";    // nf-fa-sticky_note_o
pub const IC_TERMINAL: &str = ""; // nf-oct-terminal
pub const IC_DATE: &str = "";    // nf-oct-calendar
pub const IC_LINK: &str = "";    // nf-fa-link

pub const FALLBACK_IC_NODE: &str = "[N]";
pub const FALLBACK_IC_TERMINAL: &str = "[T]";
pub const FALLBACK_IC_DATE: &str = "[D]";
pub const FALLBACK_IC_LINK: &str = "[L]";

/// Determine if Nerd Font icons should be used.
pub fn nerd_font_enabled() -> bool {
    std::env::var("PRISMX_NO_NERD_FONT").is_err()
}

pub fn terminal_icon() -> &'static str {
    if nerd_font_enabled() { IC_TERMINAL } else { FALLBACK_IC_TERMINAL }
}

