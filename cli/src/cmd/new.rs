use crate::args::{GlobalArgs, PageArgs};

use std::path::PathBuf;
pub use clap::Parser;

#[derive(Parser)]
pub struct NewCmd {
    #[clap(short='n', long)]
    pub dry_run: bool,
    #[clap(flatten)]
    global_args: GlobalArgs,
    #[clap(flatten)]
    page_args: GlobalArgs,
    pub source_path: Option<PathBuf>,
}

impl NewCmd {
    pub fn run(&self) {
        println!("Execute new command");

    }
}