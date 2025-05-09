use crate::state::TagEntry;

pub fn trust_color(source: &str) -> &'static str {
    match source {
        "manual" => "green",
        "plugin" => "yellow",
        "template" => "cyan",
        _ => "gray",
    }
}

pub fn is_trusted(tag: &TagEntry) -> bool {
    matches!(tag.source.as_str(), "manual" | "template")
}