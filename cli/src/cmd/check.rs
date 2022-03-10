pub use clap::Parser;
use super::Command;
use crate::args::GlobalArgs;
use std::fs;
use lib::{Journal, Config};

#[derive(Parser)]
pub struct CheckCmd {
    #[clap(short, long)]
    pub open: bool,
    #[clap(flatten)]
    pub global: GlobalArgs,
}

impl CheckCmd {
    pub fn run(&self) {
        self.global.init_config(); 
        let journal = Journal::new().unwrap();
        for page in journal.pages.into_iter() {
            println!("{}", page.path.display() );
        }
    }
}
