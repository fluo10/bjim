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
        let mut journal = self.global.get_journal();
        journal.data.read();
        for page in journal.data.pages.into_iter().filter_map(|page|{
            if page.has_open_task {
               Some(page)
            } else {
                None
            } }) {
            
            println!("{}", page.path.display() );
        }
    }
}
