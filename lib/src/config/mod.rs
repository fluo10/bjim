pub mod signifier;

#[cfg(test)]
mod tests;

use std::path::{Path, PathBuf};
use git2::Repository;
use dirs::config_dir;
use std::env;
use std::io;
use std::default::Default;
use serde::Deserialize;
pub use signifier::Signifier;
use toml::Value;
use std::fmt::Debug;


#[derive(Eq, PartialEq, Deserialize, Debug,)]
pub struct Config {
    pub data_dir: PathBuf,
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
    pub fn new() -> Config {
        let config: Config = Default::default();
        return config;
    }
    //fn discover(PathBuf) -> Option<Config>{
    //    config_path: config_dir()?.push("sbjo/sbjo.conf").as_path();
    //}
    
    //#[cfg(test)]
    pub fn show(&self){
        println!("{}", self.data_dir.to_str().unwrap());
    }

    pub fn from_toml(raw : &str) -> Self {
        toml::from_str(raw).unwrap()
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
