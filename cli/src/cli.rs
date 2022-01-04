pub use clap::{Parser, Subcommand};

/// Simple program to greet a person
#[derive(Parser)]
#[clap(about, version, author)]
pub struct Args {
    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    ShowTasks,
    Migrate,
}

#[derive(Parser)]
pub struct Config {
    #[clap(short, long)]
    pub path: String,
}
#[derive(Parser)]
pub struct ShowTasks {
    #[clap(short, long)]
    pub open: bool,
}

impl ShowTasks {
    pub fn run() {
        println!("Show Tasks!");
    }
}

#[derive(Parser)]
pub struct Migrate {
    #[clap(short, long)]
    pub open: bool,
}

