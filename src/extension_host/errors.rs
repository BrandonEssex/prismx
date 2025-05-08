use thiserror::Error;

#[derive(Error, Debug)]
pub enum PluginError {
    #[error("Plugin load failure")]
    LoadFailure,
}
