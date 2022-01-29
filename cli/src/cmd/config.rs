pub use clap::Parser;
use super::Command;
use std::fs;
use sbjo_lib::{Journal, Config};

#[derive(Parser)]
pub struct ConfigCmd {
    #[clap(short, long)]
    pub task_open: bool,
}

impl ConfigCmd {
    pub fn run(&self) {
        let config = Config::discover().unwrap();
        config.show();
    }
}
