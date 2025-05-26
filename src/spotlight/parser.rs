pub fn fuzzy_score(candidate: &str, query: &str) -> Option<usize> {
    let mut pos = 0;
    let cand = candidate.to_ascii_lowercase();
    let query = query.to_ascii_lowercase();
    for ch in query.chars() {
        if let Some(idx) = cand[pos..].find(ch) {
            pos += idx + 1;
        } else {
            return None;
        }
    }
    Some(pos - query.len())
}

pub fn rank<'a>(query: &str, items: &'a [&'a str]) -> Vec<&'a str> {
    let query = query.trim_start_matches('/');
    let mut scored: Vec<(usize, &str)> = items
        .iter()
        .filter_map(|c| fuzzy_score(c, query).map(|s| (s, *c)))
        .collect();
    scored.sort_by_key(|t| t.0);
    scored.into_iter().map(|t| t.1).collect()
}
