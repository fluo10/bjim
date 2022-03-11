mod check;
mod list;
mod config;
mod migrate;
mod update;

pub use check::CheckCmd;
pub use lib::{Config, Journal};
pub use list::ListCmd;
pub use config::ConfigCmd;
pub use migrate::MigrateCmd;
pub use update::UpdateCmd;


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