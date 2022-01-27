pub use clap::{Parser, Subcommand};
use super::cmd::show_tasks::ShowTasks;
use super::cmd::Command;
use super::cmd::Check;
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
    ShowTasks(ShowTasks),
    Migrate(Migrate),
    Check(Check),
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
            Commands::Check(x) => {
                x.run();
            }
        }
    }
}



#[derive(Parser)]
pub struct Migrate {
    #[clap(short, long)]
    pub open: bool,
}

