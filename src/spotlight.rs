use std::collections::HashMap;

pub fn launch_spotlight() {
    println!("[SPOTLIGHT] Command bar active. Try:");
    println!("  /theme dark");
    println!("  /plugin disable mindtrace");
    println!("  /journal");

    let commands: HashMap<&str, Box<dyn Fn()>> = HashMap::from([
        ("/theme dark", Box::new(|| println!("Theme changed to dark"))),
        ("/plugin disable mindtrace", Box::new(|| println!("mindtrace plugin disabled"))),
        ("/journal", Box::new(|| crate::zen::start_journal().unwrap())),
        ("/copy", Box::new(|| crate::clipboard::copy_node("example node"))),
        ("/workspace switch main", Box::new(|| crate::clipboard::switch_workspace("main"))),
    ]);

    use_command("/theme dark", &commands);
}

pub fn use_command(input: &str, map: &HashMap<&str, Box<dyn Fn()>>) {
    match map.get(input.trim()) {
        Some(action) => action(),
        None => println!("Unknown command: {}", input),
    }
}
