#[cfg(test)]
mod tests {
    use super::super::engine::SpotlightEngine;
    use super::super::plugin::{SearchScope, Searchable};
    use std::sync::Arc;

    struct DummyItem {
        id: String,
        text: String,
        title: String,
    }

    impl DummyItem {
        fn new(id: &str, text: &str, title: &str) -> Self {
            Self {
                id: id.into(),
                text: text.into(),
                title: title.into(),
            }
        }
    }

    impl Searchable for DummyItem {
        fn uid(&self) -> String {
            self.id.clone()
        }
        fn searchable_text(&self) -> String {
            self.text.clone()
        }
        fn display_title(&self) -> String {
            self.title.clone()
        }
        fn category(&self) -> SearchScope {
            SearchScope::Notes
        }
        fn metadata(&self) -> Option<std::collections::HashMap<String, String>> {
            None
        }
    }

    #[test]
    fn test_fuzzy_match_basic() {
        let mut engine = SpotlightEngine::new();
        let items = vec![
            Arc::new(DummyItem::new("1", "project system", "Project System")),
            Arc::new(DummyItem::new("2", "meeting notes", "Meeting Notes")),
            Arc::new(DummyItem::new("3", "internal roadmap", "Internal Roadmap")),
        ];
        engine.update_notes(items);

        let results = engine.search("meetng", SearchScope::Notes);
        assert!(!results.is_empty());
        assert_eq!(results[0].display_title, "Meeting Notes");
    }

    #[test]
    fn test_scoped_filtering() {
        let mut engine = SpotlightEngine::new();
        let note = Arc::new(DummyItem::new("1", "hello", "Note Item"));
        engine.update_notes(vec![note.clone()]);
        engine.update_projects(vec![note.clone()]);
        engine.update_todos(vec![note.clone()]);
        engine.update_plugins(vec![note.clone()]);

        assert_eq!(engine.search("hello", SearchScope::Notes).len(), 1);
        assert_eq!(engine.search("hello", SearchScope::All).len(), 4);
    }
}