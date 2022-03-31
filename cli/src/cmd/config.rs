pub use clap::Parser;
use super::super::args::{GlobalArgs, ModeArgs};



use lib::{Config};

#[derive(Parser)]
pub struct ConfigCmd {
    #[clap(flatten)]
    global_args: GlobalArgs,

    #[clap(flatten)]
    mode: ModeArgs,

}

impl ConfigCmd {
    pub fn run(&self) {
        let mut config: Config = self.global_args.to_config().unwrap();
        self.mode.add_config(&mut config);
        match config.globalize() {
            Ok(()) => {
                Config::global().show();
            },
            Err(x) => {
                eprintln!("{}",x);
            }
        }
    }
}
