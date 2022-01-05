pub use clap::{Parser, Subcommand};
use super::cmd::show_tasks::ShowTasks;
use super::cmd::Command;

/// Simple program to greet a person
#[derive(Parser)]
#[clap(about, version, author)]
pub struct Args {
    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    ShowTasks(ShowTasks),
    Migrate(Migrate),
}

impl Commands {
    pub fn run(&self){
        match &self {
            Commands::ShowTasks(x) => {
                x.run();
            }
            Commands::Migrate(x) => {
                println!("Run migrate");
            }
        }
    }
}

#[derive(Parser)]
pub struct Config {
    #[clap(short, long)]
    pub path: String,
}


#[derive(Parser)]
pub struct Migrate {
    #[clap(short, long)]
    pub open: bool,
}

