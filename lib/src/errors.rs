use std::convert::From;
use std::path::PathBuf;
use thiserror::Error;
use config::ConfigError;

#[derive(Error, Debug)]
pub enum JournalError {
    #[error("Failed to parse config: {0:?}")]
    ConfigError(ConfigError),
    #[error("transparent")]
    Other(#[from] anyhow::Error),
}

impl From<ConfigError> for JournalError {
    fn from(e: ConfigError) -> Self {
        Self::ConfigError(e)
    }
}

