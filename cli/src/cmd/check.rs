pub use clap::Parser;
use super::Command;
use std::fs;
use sbjo_lib::Journal;

#[derive(Parser)]
pub struct Check {
    #[clap(short, long)]
    pub open: bool,
}

impl Check {
    pub fn run(&self, journal: Journal) {
        for (path, page) in journal.data.pages.into_iter() {
            println!("{}", page.path.display() );
        }
    }
}
