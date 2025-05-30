use prismx::state::{AppState, SimInput};

#[test]
fn simulate_command_parses_sequence() {
    let mut state = AppState::default();
    state.spotlight_input = "/simulate enter tab delete".into();
    state.execute_spotlight_command();
    assert_eq!(state.simulate_input_queue.len(), 3);
    assert!(matches!(state.simulate_input_queue[0], SimInput::Enter));
    assert!(matches!(state.simulate_input_queue[1], SimInput::Tab));
    assert!(matches!(state.simulate_input_queue[2], SimInput::Delete));
}

#[test]
fn simulated_sequence_performs_actions() {
    let mut state = AppState::default();
    let initial = state.nodes.len();
    state.spotlight_input = "/simulate enter tab enter".into();
    state.execute_spotlight_command();
    let mut count = state.nodes.len();
    while let Some(sim) = state.simulate_input_queue.pop_front() {
        match sim {
            SimInput::Enter => {
                state.add_sibling();
                if state.nodes.len() > count {
                    state.nodes.get_mut(&state.selected.unwrap()).unwrap().label = "edited".into();
                    count = state.nodes.len();
                }
            }
            SimInput::Tab => {
                state.add_child();
                if state.nodes.len() > count {
                    state.nodes.get_mut(&state.selected.unwrap()).unwrap().label = "edited".into();
                    count = state.nodes.len();
                }
            }
            SimInput::Delete => state.delete_node(),
            SimInput::Drill => state.drill_selected(),
            SimInput::Pop => state.pop_stack(),
            SimInput::Undo => state.undo(),
            SimInput::Redo => state.redo(),
        }
    }
    assert_eq!(state.nodes.len(), initial + 3);
}
