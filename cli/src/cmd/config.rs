pub use clap::Parser;
use super::{Command, GlobalArgs};
use std::fs;
use sbjo_lib::{Journal, Config};

#[derive(Parser)]
pub struct ConfigCmd {
    #[clap(flatten)]
    global: GlobalArgs,

}

impl ConfigCmd {
    pub fn run(&self) {
        let config = match self.global.config.clone() {
            Some(x) => Config::from_path(x.as_path()).unwrap(),
            None => Config::discover().unwrap(),
        };
        config.show();
    }
}
