use std::num::ParseIntError;

use thiserror::Error;



#[derive(Debug, Error)]
pub enum HelperError {
    #[error("Unable to parse input")]
    ParseError,
    #[error("Invalid number")]
    InvalidNumber(#[from] ParseIntError)

}