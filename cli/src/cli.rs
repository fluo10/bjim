pub use clap::{Parser, Subcommand};
use std::path::PathBuf;
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
    #[clap(short, long, global=true)]
    config: Option<PathBuf>,
    #[clap(short, long, global=true)]
    journal_dir: Option<String>,
    #[clap(short, long, global=true)]
    verbose: bool,
    
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
                x.run();
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





