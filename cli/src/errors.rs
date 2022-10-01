use bjim_config::errors::ConfigError;
use bjim_lib::errors::JournalError;
use std::convert::From;

use thiserror::Error;


pub type Result<T> = std::result::Result<T, CliError>;

#[derive(Error, Debug)]
pub enum CliError {
    #[error("Error: {0:?}")]
    Message(String),
    #[error("Jounal Errhr: {0:?}")]
    JournalError(JournalError),
    #[error("transparent")]
    Other(#[from] anyhow::Error),
}

impl From<String> for CliError {
    fn from(s: String) -> CliError {
        CliError::Message(s)
    }
}

impl From<JournalError> for CliError {
    fn from(e: JournalError) -> Self {
        Self::JournalError(e)
    }
}