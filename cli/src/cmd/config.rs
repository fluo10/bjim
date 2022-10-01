use crate::args::{GlobalArgs, ModeArgs};
use crate::errors::Result;

use bjim_config::Config;

pub use clap::Parser;




#[derive(Parser)]
pub struct ConfigCmd {
    #[clap(flatten)]
    global_args: GlobalArgs,

    #[clap(flatten)]
    mode: ModeArgs,

}

impl ConfigCmd {
    pub fn run(&self) -> Result<()> {
        let mut config: Config = self.global_args.to_config()?;
        self.mode.set_config(&mut config);
        config.show();
        Ok(())
    }
}
