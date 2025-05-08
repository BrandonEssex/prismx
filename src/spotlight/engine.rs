use fuzzy_matcher::skim::SkimMatcherV2;
use fuzzy_matcher::FuzzyMatcher;

#[derive(Clone, Debug)]
pub struct SearchResult {
    pub title: String,
}

pub struct SpotlightEngine {
    matcher: SkimMatcherV2,
    entries: Vec<String>,
}

impl SpotlightEngine {
    pub fn new() -> Self {
        SpotlightEngine {
            matcher: SkimMatcherV2::default(),
            entries: vec![
                "Install Kubernetes".into(),
                "Configure nginx reverse proxy".into(),
                "Edit .zshrc for aliases".into(),
                "Restart backend service".into(),
                "Run CI tests".into(),
                "Document API usage".into(),
            ],
        }
    }

    pub fn search(&self, query: &str) -> Vec<SearchResult> {
        let mut results = self
            .entries
            .iter()
            .filter_map(|entry| {
                self.matcher
                    .fuzzy_match(entry, query)
                    .map(|score| (score, entry.clone()))
            })
            .collect::<Vec<_>>();

        results.sort_by(|a, b| b.0.cmp(&a.0));

        results
            .into_iter()
            .map(|(_, title)| SearchResult { title })
            .collect()
    }
}