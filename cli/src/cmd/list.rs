pub use clap::Parser;
use super::Command;
use std::fs;
use crate::args::GlobalArgs;
use lib::{Journal, Config};

#[derive(Parser)]
pub struct ListCmd {
    #[clap(short, long)]
    pub task_open: bool,
    #[clap(flatten)]
    pub global: GlobalArgs,
}

impl ListCmd {
    pub fn run(&self) {
        self.global.init_config();
        let mut journal = Journal::new().unwrap();
        journal.read();
        for page in journal.pages.into_iter().filter_map(|page|{
            if page.has_open_task {
               Some(page)
            } else {
                None
            } }) {
            
            println!("{}", page.path.display() );
        }
    }
}
