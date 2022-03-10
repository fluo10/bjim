use anyhow::{bail, Result};
use clap::Args;
use lib::{Config, Journal};
use std::env;
use std::path::{PathBuf};

#[derive(Args)]
pub struct GlobalArgs {
    #[clap(short, long, from_global)]
    pub config_path: Option<PathBuf>,
    #[clap(short, long, from_global)]
    pub journal_dir: Option<PathBuf>,
    #[clap(short, long, from_global)]
    pub verbose: bool,
}

impl GlobalArgs {
    pub fn init_config(&self) -> Result<()> {

        let mut config: Config = match self.config_path.as_ref() {
            Some(x) => Config::from_path(&x.as_path())?,
            None => match &self.journal_dir {
                Some(x) => Config::discover(&x)?,
                None => Config::discover(&env::current_dir().unwrap())?,
            }
        };

        if let Some(x) = &self.journal_dir {
            config.data_dir = x.clone();
        }
        config.globalize();
        Ok(())
    }
}