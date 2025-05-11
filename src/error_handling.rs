use crate::util::errors::PrismXUtilError;

pub fn handle_io_error(err: std::io::Error) -> PrismXUtilError {
    PrismXUtilError::IO(err)
}