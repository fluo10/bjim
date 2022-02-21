pub mod signifier;


use std::path::{Path, PathBuf};
use std::convert::AsRef;
use git2::Repository;
use dirs::{config_dir, home_dir};
use std::env;
use std::io;
use std::default::Default;
use serde::Deserialize;
pub use signifier::Signifier;
use toml::Value;
use std::fmt::Debug;
use std::fs::File;
use std::io::prelude::*;
use anyhow::{bail,Result};

#[derive(Eq, PartialEq, Deserialize, Debug,)]
pub struct Config {

    #[serde(default,)]
    pub data_dir: PathBuf,

    #[serde(default)]
    pub signifiers: Vec<Signifier>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
             data_dir: Repository::discover(env::current_dir().unwrap()).unwrap().workdir().unwrap().to_path_buf(),
             signifiers: Vec::new(),
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
        const LOCAL_CONFIG_NAME: &str = ".sbjo/config.toml";
        const USER_CONFIG_NAME: &str = "sbjo/config.toml";
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

    use super::{Config, Signifier};
    use std::path::PathBuf;
    
    
    #[test]
    fn parse_string_all() {
        let fromtoml = Config::from_toml(r##"data_dir = "/home/test/"
        [[signifiers]]
        name = "tag"
        emoji = "🏷️"
        [[signifiers]]
        name = "date"
        emoji = "📅"
        value = "date"
        [[signifiers]]
        name = "time"
        emoji = "⏰"
        value = "date"
        [[signifiers]]
        name = "hours"
        emoji = "⌛"
        value = "float""##);
        
        let config = Config{
            data_dir : PathBuf::from("/home/test/"),
            signifiers : vec![
                Signifier{ 
                    name: "tag".to_string(),
                    emoji: "🏷️".to_string(),
                    value: None,
                },
                Signifier{
                    name: "date".to_string(),
                    emoji: "📅".to_string(),
                    value: Some("date".to_string()),
                },
                Signifier {
                    name: "time".to_string(),
                    emoji: "⏰".to_string(),
                    value: Some("date".to_string()),
                },
                Signifier {
                    name: "hours".to_string(),
                    emoji: "⌛".to_string(),
                    value: Some("float".to_string()),
                },
            ],
        };
        assert_eq!(fromtoml, config);
    }
    
    fn parse_string_min() {
        let fromtoml = Config::from_toml(r##"data_dir = "/home/test/"##);
        
        let config = Config{
            data_dir : PathBuf::from("/home/test/"),
            signifiers : Vec::new(),
        };
        assert_eq!(fromtoml, config);
    }
}