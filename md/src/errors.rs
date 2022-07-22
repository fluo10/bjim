use thiserror::Error;

#[derive(Error, Debug)]
pub enum ParseError {
    #[error("Unexpected token")]
    InvalidToken,
    #[error("Invalid char (expected {expected:?}, found {found:?})")]
    InvalidChar {
        expected: &'static str,
        found: char
    },
    #[error("Failed to parsing token")]
    ParseTokenError,
    #[error("Token not found")]
    TokenNotFound,
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}