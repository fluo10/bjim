use thiserror::Error;

#[derive(Error, Debug)]
pub enum ParseError {
    #[error("Unexpected token")]
    InvalidToken,
    #[error("Token not found")]
    TokenNotFound,
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}