use std::fmt;

#[derive(Debug)]
pub enum PrismXUtilError {
    IO(std::io::Error),
    Serde(serde_json::Error),
    InvalidProfile(String),
}

impl fmt::Display for PrismXUtilError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PrismXUtilError::IO(err) => write!(f, "IO error: {}", err),
            PrismXUtilError::Serde(err) => write!(f, "Serialization error: {}", err),
            PrismXUtilError::InvalidProfile(msg) => write!(f, "Invalid profile: {}", msg),
        }
    }
}

impl std::error::Error for PrismXUtilError {}

impl From<std::io::Error> for PrismXUtilError {
    fn from(err: std::io::Error) -> Self {
        PrismXUtilError::IO(err)
    }
}

impl From<serde_json::Error> for PrismXUtilError {
    fn from(err: serde_json::Error) -> Self {
        PrismXUtilError::Serde(err)
    }
}