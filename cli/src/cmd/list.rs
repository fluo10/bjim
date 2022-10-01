pub use crate::errors::Result;
pub use clap::Parser;


use crate::args::GlobalArgs;
use bjim_lib::{Journal};

#[derive(Parser)]
pub struct ListCmd {
    #[clap(short, long)]
    pub task_open: bool,
    #[clap(flatten)]
    pub global_args: GlobalArgs,
}

impl ListCmd {
    pub fn run(&self) -> Result<()> {
        let config = self.global_args.to_config()?;
        let mut journal = Journal::from(config);
        journal.read();
        for page in journal.pages.into_iter().filter_map(|page|{
            if page.has_open_task {
               Some(page)
            } else {
                None
            } }) {
            
            println!("{}", page.path.display() );
        }
        Ok(())
    }
}
