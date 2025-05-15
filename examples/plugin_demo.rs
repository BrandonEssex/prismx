use prismx::plugin::Plugin;

fn main() {
    println!("Listing plugins...");
    let plugins = Plugin::list_plugins();
    for plugin in plugins {
        println!("Found plugin: {} at {}", plugin.name, plugin.path);
    }
}
