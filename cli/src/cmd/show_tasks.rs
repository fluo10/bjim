pub use clap::Parser;
use super::Command;
use std::fs;

#[derive(Parser)]
pub struct ShowTasks {
    #[clap(short, long)]
    pub open: bool,
}

impl Command for ShowTasks {
    fn run(&self) {
        show_files();
        println!("ShowTasks command");
    }
}

fn show_files(){
    let paths = fs::read_dir("./").unwrap();
    for path in paths {
        println!("{}", path.unwrap().path().display())
    }
}