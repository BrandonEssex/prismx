## Code Changes

### 1. Add Sim Queue to AppState

```rust
pub simulate_input_queue: VecDeque<SimInput>;

pub enum SimInput {
    Enter,
    Tab,
    Delete,
    Drill,
    Pop,
    Undo,
    Redo,
    // Add more as needed
}
2. Add /simulate Command Handler
In Spotlight command parser:

if input.starts_with("/simulate") {
    let sequence = input.strip_prefix("/simulate").unwrap().trim();
    for token in sequence.split_whitespace() {
        let mapped = match token.to_lowercase().as_str() {
            "enter" => SimInput::Enter,
            "tab" => SimInput::Tab,
            "delete" => SimInput::Delete,
            "drill" => SimInput::Drill,
            "pop" => SimInput::Pop,
            "undo" => SimInput::Undo,
            "redo" => SimInput::Redo,
            _ => continue,
        };
        state.simulate_input_queue.push_back(mapped);
    }
}
3. Inject Simulated Input Into Frame Tick Loop
In the main update/render loop:

if let Some(sim_input) = state.simulate_input_queue.pop_front() {
    match sim_input {
        SimInput::Enter => state.add_sibling(),
        SimInput::Tab => state.add_child(),
        SimInput::Delete => state.delete_node(),
        SimInput::Drill => state.drill_selected(),
        SimInput::Pop => state.pop_stack(),
        SimInput::Undo => state.undo(),
        SimInput::Redo => state.redo(),
    }

    if state.debug_input_mode {
        eprintln!("🧪 Simulated input: {:?}", sim_input);
    }
}
4. Optional: Show Summary on Completion
if state.simulate_input_queue.is_empty() && state.debug_input_mode {
    eprintln!("🧪 Simulation complete.");
}
