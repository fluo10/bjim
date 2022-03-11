pub use clap::{Parser, Subcommand};
use std::path::PathBuf;

use crate::cmd::CheckCmd;
use crate::cmd::ConfigCmd;
use crate::cmd::ListCmd;
use crate::cmd::MigrateCmd;
use crate::cmd::UpdateCmd;




/// Simple program to greet a person
#[derive(Parser)]
#[clap(about, version, author)]
pub struct Args {
    #[clap(short, long, global=true)]
    config_path: Option<PathBuf>,
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
    Update(UpdateCmd),
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
            Commands::Update(x) => {
                x.run();
            }
        }
    }
}





