use fuzzy_matcher::skim::SkimMatcherV2;
use fuzzy_matcher::FuzzyMatcher;

#[derive(Clone)]
pub struct SearchResult {
    pub uid: String,
    pub display_title: String,
    pub score: i64,
}

pub struct SpotlightEngine {
    matcher: SkimMatcherV2,
    items: Vec<SearchResult>,
}

impl SpotlightEngine {
    pub fn new() -> Self {
        SpotlightEngine {
            matcher: SkimMatcherV2::default(),
            items: vec![
                SearchResult {
                    uid: "1".into(),
                    display_title: "Welcome to PrismX".into(),
                    score: 100,
                },
                SearchResult {
                    uid: "2".into(),
                    display_title: "Getting Started Guide".into(),
                    score: 90,
                },
                SearchResult {
                    uid: "3".into(),
                    display_title: "Plugin API Overview".into(),
                    score: 80,
                },
            ],
        }
    }

    pub fn search(&self, query: &str) -> Vec<SearchResult> {
        let mut results = vec![];

        for item in &self.items {
            if let Some((score, _)) = self.matcher.fuzzy_match(&item.display_title, query).map(|s| (s, ())) {
                let mut matched = item.clone();
                matched.score = score;
                results.push(matched);
            }
        }

        results.sort_by(|a, b| b.score.cmp(&a.score));
        results
    }
}