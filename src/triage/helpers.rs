use chrono::NaiveDate;
use regex::Regex;
use super::state::TriageEntry;

/// Extract `#TAG` tokens from text.
pub fn extract_tags(text: &str) -> Vec<String> {
    let re = Regex::new(r"#\w+").unwrap();
    re.find_iter(text)
        .map(|m| m.as_str().to_string())
        .collect()
}

/// Sort triage entries by priority tags.
/// `#NOW` > `#TRITON` > `#TODO`/`#PRIORITY` > others.
pub fn sort_by_priority(entries: &mut [TriageEntry]) {
    fn priority(entry: &TriageEntry) -> u8 {
        if entry.tags.iter().any(|t| t == "#NOW") {
            0
        } else if entry.tags.iter().any(|t| t == "#TRITON") {
            1
        } else if entry.tags.iter().any(|t| t == "#TODO" || t == "#PRIORITY") {
            2
        } else {
            3
        }
    }
    entries.sort_by_key(|e| priority(e));
}

/// Parse a YYYY-MM-DD date string from text.
pub fn parse_due_date(text: &str) -> Option<NaiveDate> {
    let re = Regex::new(r"(\d{4}-\d{2}-\d{2})").unwrap();
    re.captures(text)
        .and_then(|cap| NaiveDate::parse_from_str(&cap[1], "%Y-%m-%d").ok())
}
