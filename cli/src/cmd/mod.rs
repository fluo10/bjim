pub mod check;
pub mod list;
pub mod config;
pub mod migrate;
pub use check::CheckCmd;
use lib::{Config, Journal};
use std::io::{Error, Result, ErrorKind};
pub use list::ListCmd;
pub use config::ConfigCmd;
pub use migrate::MigrateCmd;
use std::path::PathBuf;

pub use clap::Args;


pub trait Sub {
    /*fn get_journal(&self) -> Result<Config> {

    }
    */
    fn run(&self);
    fn get_config(&self){
        
    }
    fn get_journal(&self);

}

pub trait Command {


}