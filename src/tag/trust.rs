use std::collections::HashSet;

pub fn is_tag_trusted(tag: &str, trusted: &HashSet<String>) -> bool {
    trusted.contains(tag)
}