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

#[derive(Args)]
struct GlobalArgs {
    #[clap(short, long, from_global)]
    config: Option<PathBuf>,
    #[clap(short, long, from_global)]
    journal_dir: Option<String>,
    #[clap(short, long, from_global)]
    verbose: bool,
}


pub trait Command {}