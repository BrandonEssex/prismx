#[cfg(test)]
mod tests {
    use crate::spotlight::engine::SpotlightEngine;

    #[test]
    fn test_basic_search_results() {
        let engine = SpotlightEngine::new();
        let results = engine.search("demo");
        assert!(results.is_empty(), "Expected empty result on fresh engine");
    }

    #[test]
    fn test_query_state_updates() {
        let mut state = crate::spotlight::state::SpotlightState::new();
        state.update_query('d');
        state.update_query('e');
        state.update_query('m');
        state.update_query('o');
        assert_eq!(state.query, "demo");
    }

    #[test]
    fn test_selection_navigation_bounds() {
        let mut state = crate::spotlight::state::SpotlightState::new();
        state.move_up(); // should not go below 0
        assert_eq!(state.selected, 0);
    }
}