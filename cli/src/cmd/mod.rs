pub mod check;
pub mod list;
pub mod config;
pub mod migrate;
pub use check::CheckCmd;
pub use sbjo_lib::{Config, Journal};
use std::io::{Error, Result, ErrorKind};
pub use list::ListCmd;
pub use config::ConfigCmd;
pub use migrate::MigrateCmd;


pub trait Command {
    /*fn get_journal(&self) -> Result<Config> {

    }
    */
    fn run(&self);
}

pub trait Filter {

}