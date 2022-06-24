use thiserror::Error;

#[derive(Error, Debug)]
pub enum ParseError {
    #[error("Unexpected token")]
    UnexpectedTokens,
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}