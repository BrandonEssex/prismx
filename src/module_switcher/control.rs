#[derive(Debug, Clone)]
pub enum Module {
    GemX,
    Zen,
    Triage,
    Settings,
}

pub fn switch_module(name: &str) -> Option<Module> {
    match name {
        "gemx" => Some(Module::GemX),
        "zen" => Some(Module::Zen),
        "triage" => Some(Module::Triage),
        "settings" => Some(Module::Settings),
        _ => None,
    }
}
