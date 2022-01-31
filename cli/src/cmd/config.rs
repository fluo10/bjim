pub use clap::Parser;
use super::super::args::GlobalArgs;
use std::fs;
use lib::{Journal, Config};

#[derive(Parser)]
pub struct ConfigCmd {
    #[clap(flatten)]
    global: GlobalArgs,

}

impl ConfigCmd {
    pub fn run(&self) {
        let config = self.global.get_config();
        config.show();
    }
}
