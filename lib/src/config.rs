mod template;
mod tag;

pub use tag::TagConfig;
use template::RegularLogTemplate;

use std::path::{Path, PathBuf};
use std::collections::HashMap;
use std::convert::{AsRef, TryFrom};


use std::default::Default;
use std::fmt::Debug;
use std::fs::File;
use std::io::prelude::*;

use anyhow::{bail,Error, Result};
use dirs::{config_dir, home_dir};

use once_cell::sync::OnceCell;
use serde::{Deserialize, Serialize};


static INSTANCE: OnceCell<Config> = OnceCell::new();

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

#[derive(Deserialize, Serialize, Debug,)]
pub struct Config {

    #[serde(default,)]
    pub data_dir: PathBuf,

    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub tags: HashMap<String, TagConfig>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub templates: Vec<RegularLogTemplate>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
             data_dir: PathBuf::from("."),
             tags: HashMap::new(),
             templates: Vec::new(),
        }
    }
}

impl Config {
    pub fn new() -> Self {
        let config: Config = Default::default();
        return config;
    }

    pub fn from_path(path: &dyn AsRef<Path>) -> Result<Self> {
        let mut f = File::open(path)?;
        let mut contents = String::new();
        f.read_to_string(&mut contents)
            .expect("something went wrong reading the file");
        Ok(Self::try_from(contents.as_str())?)
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
        let mut config: Config = Config::from_path(&config_path)?;
        if config.data_dir.as_path() == PathBuf::from(".") {
            config.data_dir = path.as_ref().to_path_buf();
        }
        Ok(config)
    }
    pub fn from_path_and_journal_dir(path: &dyn AsRef<Path>, journal_dir: &dyn AsRef<Path>) -> Result<Config> {
        let mut config: Config = Config::from_path(path)?;
        config.data_dir = journal_dir.as_ref().to_path_buf();
        Ok(config)
    }
    pub fn from_user_config() -> Result<Config> {
        let path: PathBuf = get_user_config_path().ok_or(anyhow::anyhow!("User config not found"))?;
        Ok(Config::from_path(&path)?)
    }
    pub fn global() -> &'static Config {
        INSTANCE.get().expect("Config is not initialized")
    }
    pub fn globalize(self) -> Result<()> {
        match INSTANCE.set(self) {
            Ok(()) => Ok(()),
            Err(_x) => bail!("Failed to globalize config"),
        }
    }
    pub fn show(&self){
        
        println!("{}", self.to_string().unwrap());
    }
    pub fn to_string(&self) -> Result<String> {
        let result = toml::to_string(self)?;
        Ok(result)
    }
}

impl TryFrom<&str> for Config {
    type Error = Error;
    fn try_from(raw : &str) -> Result<Self> {
        let mut config:Self = toml::from_str(raw)?;
        if config.data_dir.starts_with("~/") {
            
            let leaf = config.data_dir.strip_prefix("~").unwrap().to_str();
            config.data_dir = home_dir().unwrap().join(leaf.unwrap());
        }
        Ok(config)
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
        let mut config = Config{
            data_dir : PathBuf::from("/home/test/"),
            ..Config::default()
        };
        config.tags.insert(String::from("default"), TagConfig::default());
        assert_parse(
            r##"data_dir = "/home/test/"
[tags.default]
"##,
            config
        );
    }
    
    fn parse_string_min() {
        let fromtoml = Config::try_from(r##"data_dir = "/home/test/"##);
        
        let config = Config{
            data_dir : PathBuf::from("/home/test/"),
            ..Config::default()
        };
        assert_eq!(r##"data_dir = "/home/test/"##, config);
    }
}