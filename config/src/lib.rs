mod collection;
mod core;
mod errors;
mod format;
mod period;
mod tag;

pub use tag::TagConfig;
pub use collection::CollectionConfig;
pub use self::core::CoreConfig;
pub use format::PeriodFormat;
pub use errors::ConfigError;
pub use period::Period;

use std::ffi::OsString;
use std::path::{Path, PathBuf};
use std::collections::{HashMap, HashSet};
use std::convert::{AsRef, TryFrom};


use std::default::Default;
use std::fmt::Debug;
use std::fs::File;
use std::io::prelude::*;

use anyhow::{bail,Error};
use dirs::{config_dir, home_dir};

use serde::{Deserialize, Serialize};

type Result<T> = std::result::Result<T, ConfigError>;

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

#[derive(Deserialize, Serialize, Debug, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Config {

    #[serde(default,)]
    pub core: CoreConfig,

    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub tags: HashMap<String, TagConfig>,

    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub collections: HashMap<String, CollectionConfig>,

}

impl Default for Config {
    fn default() -> Self {
        Self {
            core: CoreConfig{
                data_dir: PathBuf::from("."),
                read_only: false,
                index_file_names: HashSet::new(),
            },
            tags: HashMap::new(),
            collections: HashMap::new(),
        }
    }
}

impl Config {
    pub fn new() -> Self {
        let config: Config = Default::default();
        return config;
    }

    /*
    pub fn discover(path: &AsRef<Path>) -> Result<Self>{
        const LOCAL_CONFIG_NAME: &str = ".bjim/config.toml";
        const USER_CONFIG_NAME: &str = "bjim/config.toml";
        let mut local_config_path: PathBuf =  Repository::discover(path.as_ref()).unwrap().workdir().unwrap().to_path_buf();
        local_config_path.push(LOCAL_CONFIG_NAME);

        let mut user_config_path:PathBuf = config_dir().unwrap();
        user_config_path.push(USER_CONFIG_NAME);

        if local_config_path.is_file() {
            Self::from_path(local_config_path.as_path())
        } else if user_config_path.is_file() {
            Self::from_path(user_config_path.as_path())
        } else {
            bail!("Not found")
        }
    }
    */

    pub fn from_journal_dir(path: &dyn AsRef<Path>) -> Result<Config> {
        let config_path = get_local_config_path(path);
        let mut config: Config = config_path.as_path().try_into()?;
        if config.core.data_dir.as_path() == PathBuf::from(".") {
            config.core.data_dir = path.as_ref().to_path_buf();
        }
        Ok(config)
    }
    pub fn from_path_and_journal_dir(path: &dyn AsRef<Path>, journal_dir: &dyn AsRef<Path>) -> Result<Config> {
        let mut config: Config = path.as_ref().try_into()?;
        config.core.data_dir = journal_dir.as_ref().to_path_buf();
        Ok(config)
    }
    pub fn from_user_config() -> Result<Config> {
        let path: PathBuf = get_user_config_path().ok_or(ConfigError::NotFound(None))?;
        Config::try_from(&path)
    }

    pub fn show(&self){
        
        println!("{}", self.to_string().unwrap());
    }
    pub fn to_string(&self) -> Result<String> {
        toml::to_string(self).map_err(|e| Error::new(e).into())
    }

}
impl TryFrom<&str> for Config {
    type Error = ConfigError;
    fn try_from(raw : &str) -> Result<Self> {
        let mut config:Self = toml::from_str(raw)?;
        if config.core.data_dir.starts_with("~/") {
            let leaf = config.core.data_dir.strip_prefix("~").unwrap().to_str();
            config.core.data_dir = home_dir().unwrap().join(leaf.unwrap());
        }
        Ok(config)
    }
}
impl TryFrom<&PathBuf> for Config {
    type Error = ConfigError;
    fn try_from(path: &PathBuf) -> Result<Config> {
        path.as_path().try_into()
    }
}
impl TryFrom<&Path> for Config {
    type Error = ConfigError;
    fn try_from(path: &Path) -> Result<Config> {
        let mut f = File::open(path)?;
        let mut contents = String::new();
        f.read_to_string(&mut contents)
            .expect("something went wrong reading the file");
        Self::try_from(contents.as_str())
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    fn assert_parse(s: &str, c: Config) {
        assert_eq!(
            Config::try_from(s).unwrap(),
            c
        );
    }
    
    #[test]
    fn parse_string_all() {
        let config: Config = Config{
            core: CoreConfig {
                data_dir: PathBuf::from("."),
                ..Default::default()
            },
            tags: HashMap::from([
                (
                    "Routine".to_string(),
                    TagConfig{
                        repeat: true,
                        ..TagConfig::default()
                    }
                ),
            ]),
            collections: HashMap::from([
                (
                    "Dailylog".to_string(),
                    CollectionConfig{
                        path: Some(PeriodFormat::try_from("dailylog/%Y/%m/%d").unwrap()),
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
[collections.Dailylog]
auto_migration = true
path = "dailylog/%Y/%m/%d""#;
        assert_parse(
            toml,
            config
        );
    }
    
    fn parse_string_min() {
        
        let config = Config{
            core: CoreConfig{
                data_dir : PathBuf::from("/home/test/"),
                ..Default::default()
            },
            ..Config::default()
        };
        assert_parse(r##"data_dir = "/home/test/"##, config);
    }
}
