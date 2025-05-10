use fuzzy_matcher::skim::SkimMatcherV2;
use fuzzy_matcher::FuzzyMatcher;

#[derive(Debug)]
pub struct SearchResult {
    pub index: usize,
    pub score: i64,
}

#[derive(Debug)]
pub struct SpotlightEngine {
    matcher: SkimMatcherV2,
}

impl SpotlightEngine {
    pub fn new() -> Self {
        Self {
            matcher: SkimMatcherV2::default(),
        }
    }

    pub fn search<'a>(&self, query: &str, candidates: &'a [String]) -> Vec<SearchResult> {
        candidates.iter()
            .enumerate()
            .filter_map(|(i, text)| {
                self.matcher.fuzzy_match(text, query)
                    .map(|score| SearchResult { index: i, score })
            })
            .collect()
    }
}