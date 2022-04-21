use std::convert::From;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("Config not found: {0:?}")]
    OpenFailed(std::io::Error),
    #[error("Failed to parse config: {0:?}")]
    ParseFailed(toml::de::Error),
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

