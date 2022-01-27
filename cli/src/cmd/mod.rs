//pub mod config;
pub mod show_tasks;
//pub mod migrate;
pub mod check;
pub use check::Check;
pub use sbjo_lib::{Config, Journal};
use std::io::{Error, Result, ErrorKind};


pub trait Command {
    /*fn get_journal(&self) -> Result<Config> {

    }
    */
    fn run(&self);
}

pub trait Filter {

}