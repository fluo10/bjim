use std::path::{Path, PathBuf};
use git2::Repository;
use dirs::config_dir;
use std::env;
use std::io;
use std::default::Default;

pub struct Config {
    pub data_dir: PathBuf,
}

impl Default for Config {
    fn default() -> Self {
        Self {
             data_dir: Repository::discover(env::current_dir().unwrap()).unwrap().workdir().unwrap().to_path_buf(),
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
