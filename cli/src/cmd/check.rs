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
        
        let journal = self.global.get_journal();
        for page in journal.data.pages.into_iter() {
            println!("{}", page.path.display() );
        }
    }
}
