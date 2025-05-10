use crate::tag::TagEntry;

pub fn trust_color(source: &str) -> &'static str {
    match source {
        "internal" => "green",
        "external" => "yellow",
        "unknown" => "gray",
        _ => "blue",
    }
}

pub fn is_trusted(tag: &TagEntry) -> bool {
    tag.source == "internal"
}