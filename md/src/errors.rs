use thiserror::Error;

#[derive(Error, Debug)]
pub enum ParseError {
    #[error("Unexpected token")]
    InvalidToken,
    #[error("Failed to parsing token")]
    ParseTokenError,
    #[error("Token not found")]
    TokenNotFound,
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}