use clap::Args;
use lib::{Config, Journal};
use std::env;
use std::path::{PathBuf};

#[derive(Args)]
pub struct GlobalArgs {
    #[clap(short, long, from_global)]
    config_path: Option<PathBuf>,
    #[clap(short, long, from_global)]
    journal_dir: Option<PathBuf>,
    #[clap(short, long, from_global)]
    verbose: bool,
}

impl GlobalArgs {
    pub fn get_config(&self) -> Config {
        match (self.config_path.clone(), self.journal_dir.clone()) {
            (Some(x), Some(y)) => {
                let mut config = Config::from_path(&x).unwrap();
                config.data_dir = y;
                config
            },
            (Some(x), None) => {
                Config::from_path(&x).unwrap()
            }
            (None, Some(x)) => {
                Config::discover(&x).unwrap()
            },
            (None, None) => {
                Config::discover(&env::current_dir().unwrap()).unwrap()
            },
        }
    }
    pub fn get_journal(&self) -> Journal {
        Journal::from_config(self.get_config()).unwrap()
    }
}