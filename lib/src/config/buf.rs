use super::tag::TagConfig;
use super::template::{RegularLogTemplate, RegularPathFormat};

use std::path::{Path, PathBuf};
use std::collections::HashMap;
use std::convert::{AsRef, TryFrom};


use std::default::Default;
use std::fmt::Debug;
use std::fs::File;
use std::io::prelude::*;

use anyhow::{bail,Error, Result};
use dirs::{config_dir, home_dir};

use serde::{Deserialize, Serialize};

pub fn get_local_config_path(path: &dyn AsRef<Path>) -> PathBuf {
    let mut path: PathBuf = path.as_ref().to_path_buf();
    path.push(".bjim");
    path.push("config.toml");
    path
}
pub fn get_user_config_path() -> Option<PathBuf> {
    let mut path :PathBuf = config_dir()?;
    path.push("bjim");
    path.push("config.toml");
    Some(path)
}

/// Temporally config data
#[derive(Deserialize, Serialize, Debug, PartialEq, Default)]
pub struct ConfigBuf {

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data_dir: Option<PathBuf>,

    #[serde(skip_deserializing, skip_serializing_if = "Option::is_none")]
    pub dry_run: Option<bool>,

    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub tags: HashMap<String, TagConfig>,

    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub templates: HashMap<String, RegularLogTemplate>,

}

impl ConfigBuf {
    pub fn new() -> Self {
        let config: ConfigBuf = Default::default();
        return config;
    }

    pub fn from_path(path: &dyn AsRef<Path>) -> Result<Self> {
        let mut f = File::open(path)?;
        let mut contents = String::new();
        f.read_to_string(&mut contents)
            .expect("something went wrong reading the file");
        Ok(Self::try_from(contents.as_str())?)
    }
    pub fn merge_with(&mut self, other: &Self) {
        todo!();
    }

    pub fn from_journal_dir(path: &dyn AsRef<Path>) -> Result<Self> {
        let config_path = get_local_config_path(path);
        let mut config: ConfigBuf = ConfigBuf::from_path(&config_path)?;
        if config.data_dir.is_none() {
            config.data_dir = Some(path.as_ref().to_path_buf());
        }
        Ok(config)
    }
    pub fn from_path_and_journal_dir(path: &dyn AsRef<Path>, journal_dir: &dyn AsRef<Path>) -> Result<Self> {
        let mut config: ConfigBuf = ConfigBuf::from_path(path)?;
        config.data_dir = Some(journal_dir.as_ref().to_path_buf());
        Ok(config)
    }
    pub fn from_user_config() -> Result<Self> {
        let path: PathBuf = get_user_config_path().ok_or(anyhow::anyhow!("User config not found"))?;
        Ok(Self::from_path(&path)?)
    }
    pub fn to_string(&self) -> Result<String> {
        let result = toml::to_string(self)?;
        Ok(result)
    }
}

impl TryFrom<&str> for ConfigBuf {
    type Error = Error;
    fn try_from(raw : &str) -> Result<Self> {
        let mut config:Self = toml::from_str(raw)?;
        Ok(config)
    }
}

#[cfg(test)]
mod tests {

    use super::*; 

    fn assert_parse(s: &str, c: ConfigBuf) {
        assert_eq!(
            ConfigBuf::try_from(s).unwrap(),
            c
        );
    }
    
    #[test]
    fn parse_string_all() {
        let mut config: ConfigBuf = ConfigBuf{
            data_dir: Some(PathBuf::from(".")),
            tags: HashMap::from([
                (
                    "Routine".to_string(),
                    TagConfig{
                        repeat: true,
                        ..TagConfig::default()
                    }
                ),
            ]),
            templates: HashMap::from([
                (
                    "Dailylog".to_string(),
                    RegularLogTemplate{
                        path_format: Some(RegularPathFormat::try_from("dailylog/%Y/%m/%d").unwrap()),
                        auto_migration: true,
                        ..Default::default()
                    }
                )
            ]),
            ..Default::default()
        };
        let toml = r#"
data_dir = "."
[tags.Routine]
repeat = true
inherit = true
migrate = true
[templates.Dailylog]
auto_migration = true
path_format = "dailylog/%Y/%m/%d""#;
        assert_parse(
            toml,
            config
        );
    }
    

}