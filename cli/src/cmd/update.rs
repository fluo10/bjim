use crate::args::{GlobalArgs, ModeArgs};
use lib::{Config, Journal};

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
    pub fn run(&self) {
        match self.global_args.to_config().unwrap().globalize(){
            Ok(()) => {
                Config::global().show();
            },
            Err(x) => {
                eprintln!("{}",x);
            }
        }
        let mut journal: Journal = Journal::new().unwrap();
        journal.reload();
        journal.update();

    }
}