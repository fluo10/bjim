pub use clap::Parser;
use super::Command;
use std::fs;
use sbjo_lib::{Journal, Config};

#[derive(Parser)]
pub struct ListCmd {
    #[clap(short, long)]
    pub task_open: bool,
}

impl ListCmd {
    pub fn run(&self) {
        let config = Config::discover().unwrap();
        let mut journal = Journal::from_config(config).unwrap();
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
