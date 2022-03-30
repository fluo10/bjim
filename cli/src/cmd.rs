mod check;
mod config;
mod list;
mod migrate;
mod template;
mod update;

use check::CheckCmd;
use config::ConfigCmd;
use lib::{Config, Journal};
use list::ListCmd;
use migrate::MigrateCmd;
use template::TemplateCmd;
use update::UpdateCmd;

use std::path::PathBuf;

use clap::{Args, Parser, Subcommand};

/// Simple program to greet a person
#[derive(Parser)]
#[clap(about, version, author)]
pub struct Cmd {
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
    Template(TemplateCmd),
    Update(UpdateCmd),
}

impl Cmd {
    pub fn run(&self){
        match &self.command {
            Commands::Check(x) => {
                x.run();
            }
            Commands::Config(x) => {
                x.run();
            }
            Commands::List(x) => {
                x.run();
            }
            Commands::Migrate(x) => {
                x.run();
            }
            Commands::Template(x) => {
                x.run();
            }
            Commands::Update(x) => {
                x.run();
            }
        }
    }
}
