use std::fmt;

#[derive(Debug)]
pub enum PrismXUtilError {
    InvalidFormat,
    MissingField(String),
    IOError(std::io::Error),
}

impl fmt::Display for PrismXUtilError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PrismXUtilError::InvalidFormat => write!(f, "Invalid format"),
            PrismXUtilError::MissingField(field) => write!(f, "Missing field: {}", field),
            PrismXUtilError::IOError(e) => write!(f, "IO error: {}", e),
        }
    }
}

impl std::error::Error for PrismXUtilError {}