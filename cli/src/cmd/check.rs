use crate::errors::{CliError, Result};

use clap::Parser;

use crate::args::GlobalArgs;

use bjim_lib::{Journal};

#[derive(Parser)]
pub struct CheckCmd {
    #[clap(short, long)]
    pub open: bool,
    #[clap(flatten)]
    pub global_args: GlobalArgs,
}

impl CheckCmd {
    pub fn run(&self) -> Result<()> {
        let config = self.global_args.to_config()?; 
        let journal = Journal::new().unwrap();
        for page in journal.pages.into_iter() {
            println!("{}", page.path.display() );
        }
        Ok(())
    }
}
