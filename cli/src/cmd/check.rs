pub use clap::Parser;

use crate::args::GlobalArgs;

use lib::{Journal};

#[derive(Parser)]
pub struct CheckCmd {
    #[clap(short, long)]
    pub open: bool,
    #[clap(flatten)]
    pub global_args: GlobalArgs,
}

impl CheckCmd {
    pub fn run(&self) {
        self.global_args.to_config().unwrap().globalize(); 
        let journal = Journal::new().unwrap();
        for page in journal.pages.into_iter() {
            println!("{}", page.path.display() );
        }
    }
}
