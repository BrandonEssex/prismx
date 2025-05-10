use crate::util::errors::PrismXUtilError;

pub fn map_util_error(err: std::io::Error) -> PrismXUtilError {
    PrismXUtilError::IoError(err)
}