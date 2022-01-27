pub use clap::Parser;
use super::Command;
use std::fs;
use sbjo_lib::{Journal, Config};

#[derive(Parser)]
pub struct Check {
    #[clap(short, long)]
    pub open: bool,
}

impl Check {
    pub fn run(&self) {
        let config = Config::discover().unwrap();
        let mut journal = Journal::from_config(config).unwrap();
        for (path, page) in journal.data.pages.into_iter() {
            println!("{}", page.path.display() );
        }
    }
}
