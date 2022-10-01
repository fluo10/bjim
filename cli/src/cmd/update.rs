use crate::args::{GlobalArgs, ModeArgs};
use crate::errors::Result;

use bjim_config::{Config};
use bjim_lib::{Journal};


use clap::Parser;

/// Subcommand for update journal
#[derive(Parser)]
pub struct UpdateCmd {

    #[clap(flatten)]
    pub global_args: GlobalArgs,

    #[clap(flatten)]
    pub mode_args: ModeArgs,

    #[clap(long)]
    pub push: bool,

    #[clap(long)]
    pub pull: bool,

}

impl UpdateCmd {
    pub fn run(&self) -> Result<()> {
        let config = self.global_args.to_config()?;
        let mut journal: Journal = Journal::from(config);
        journal.reload();
        journal.update();
        Ok(())

    }
}