use crate::tag::TagEntry;

pub fn trust_color(source: &str) -> &'static str {
    match source {
        "verified" => "green",
        "unverified" => "yellow",
        "flagged" => "red",
        _ => "gray",
    }
}

pub fn is_trusted(tag: &TagEntry) -> bool {
    matches!(tag.trust.as_deref(), Some("verified"))
}