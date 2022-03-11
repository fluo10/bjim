pub use clap::Parser;
use super::super::args::GlobalArgs;

use lib::{Config};

#[derive(Parser)]
pub struct ConfigCmd {
    #[clap(flatten)]
    global: GlobalArgs,

}

impl ConfigCmd {
    pub fn run(&self) {
        match self.global.init_config(){
            Ok(()) => {
                Config::global().show();
            },
            Err(x) => {
                eprintln!("{}",x);
            }
        }
    }
}
