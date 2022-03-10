mod template;
mod tag;

pub use tag::TagConfig;

use std::path::{Path, PathBuf};
use std::collections::HashMap;
use std::convert::AsRef;
use std::env;
use std::io;
use std::default::Default;
use std::fmt::Debug;
use std::fs::File;
use std::io::prelude::*;

use anyhow::{bail,Result};
use dirs::{config_dir, home_dir};
use git2::Repository;
use once_cell::sync::OnceCell;
use serde::Deserialize;
use toml::Value;

static INSTANCE: OnceCell<Config> = OnceCell::new();

#[derive(PartialEq, Deserialize, Debug,)]
pub struct Config {

    #[serde(default,)]
    pub data_dir: PathBuf,

    #[serde(default)]
    pub tags: HashMap<String, TagConfig>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
             data_dir: Repository::discover(env::current_dir().unwrap()).unwrap().workdir().unwrap().to_path_buf(),
             tags: HashMap::new(),
        }
    }
}

impl Config {
    pub fn new() -> Self {
        let config: Config = Default::default();
        return config;
    }

    pub fn from_path(path: &Path) -> Result<Self> {
        let mut f = File::open(path).expect("file not found");
        let mut contents = String::new();
        f.read_to_string(&mut contents)
            .expect("something went wrong reading the file");
        Ok(Self::from_toml(contents.as_str()))
    }

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
    pub fn global() -> &'static Config {
        INSTANCE.get().expect("Config is not initialized")
    }
    pub fn globalize(self) {
        INSTANCE.set(self).unwrap();
    }
    //#[cfg(test)]
    pub fn show(&self){
        println!("{}", self.data_dir.to_str().unwrap());
    }

    pub fn from_toml(raw : &str) -> Self {
        let mut config:Self = toml::from_str(raw).unwrap();
        if config.data_dir.starts_with("~/") {
            
            let leaf = config.data_dir.strip_prefix("~").unwrap().to_str();
            config.data_dir = home_dir().unwrap().join(leaf.unwrap());
        }
        config
    }




    
//    pub fn new( path: Option<Path> ) => Config {
//        match path {
//            Some(path) => 
//        date: Option<date>,
//        
//        println!(dirs::config_dir)
//
//        .config/sbjo/sbjo.conf
//        todo!;
//    }
}

#[cfg(test)]
mod tests {

    use super::*; 
    
    #[test]
    fn parse_string_all() {
        let fromtoml = Config::from_toml(r##"data_dir = "/home/test/"
[tags.default]
"##);
        let mut config = Config{
            data_dir : PathBuf::from("/home/test/"),
            tags : HashMap::new(),
        };
        config.tags.insert(String::from("default"), TagConfig::default());
        assert_eq!(fromtoml, config);
    }
    
    fn parse_string_min() {
        let fromtoml = Config::from_toml(r##"data_dir = "/home/test/"##);
        
        let config = Config{
            data_dir : PathBuf::from("/home/test/"),
            tags : HashMap::new(),
        };
        assert_eq!(fromtoml, config);
    }
}