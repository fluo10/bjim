use std::convert::From;
use std::path::PathBuf;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("File missing: {0:?}")]
    NotFound(Option<PathBuf>),
    #[error("Config not found: {0:?}")]
    OpenFailed(std::io::Error),
    #[error("Failed to parse config: {0:?}")]
    ParseFailed(toml::de::Error),
    #[error("infallible")]
    Infallible(std::convert::Infallible),
    #[error("transparent")]
    Other(#[from] anyhow::Error),
}

impl From<std::io::Error> for ConfigError {
    fn from(e: std::io::Error) -> Self {
        ConfigError::OpenFailed(e)
    }
}

impl From<toml::de::Error> for ConfigError {
    fn from(e: toml::de::Error) -> Self {
        ConfigError::ParseFailed(e)
    }
}

