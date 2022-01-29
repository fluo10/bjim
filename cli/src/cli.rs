pub use clap::{Parser, Subcommand};
use super::cmd::Command;
use super::cmd::CheckCmd;
use super::cmd::ConfigCmd;
use super::cmd::ListCmd;
use super::cmd::MigrateCmd;
pub use sbjo_lib::{Config, Journal};

/// Simple program to greet a person
#[derive(Parser)]
#[clap(about, version, author)]
pub struct Args {
    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Check(CheckCmd),
    Config(ConfigCmd),
    Migrate(MigrateCmd),
    List(ListCmd),
}

impl Commands {
    pub fn run(&self){
        match &self {
            Commands::Config(x) => {
                x.run();
            }
            Commands::Migrate(x) => {
                println!("Run migrate");
            }
            Commands::Check(x) => {
                x.run();
            }
            Commands::List(x) => {
                x.run();
            }
        }
    }
}





