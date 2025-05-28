use super::{commands::COMMANDS, parser};

/// Compute a fuzzy match score. Delegates to [`parser::fuzzy_score`].
pub fn fuzzy_score(candidate: &str, query: &str) -> Option<usize> {
    parser::fuzzy_score(candidate, query)
}

/// Return command suggestions using the predictive ranking parser.
pub fn command_suggestions(input: &str) -> Vec<&'static str> {
    parser::rank(input.trim_start_matches('/'), &COMMANDS)
        .into_iter()
        .take(5)
        .collect()
}

/// Return command suggestions along with fuzzy match scores.
pub fn command_suggestions_scored(input: &str) -> Vec<(&'static str, usize)> {
    parser::rank_with_scores(input.trim_start_matches('/'), &COMMANDS)
        .into_iter()
        .take(5)
        .collect()
}
