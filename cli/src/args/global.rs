use anyhow::{Error, Result};
use clap::Args;
use git2::Repository;
use bjim_config::{Config};
use std::env;
use std::path::{PathBuf};
use std::convert::{TryFrom, TryInto};

#[derive(Args, Clone)]
pub struct GlobalArgs {
    #[clap(short, long, from_global)]
    pub config_path: Option<PathBuf>,
    #[clap(short, long, from_global)]
    pub journal_dir: Option<PathBuf>,
    #[clap(short, long, from_global)]
    pub verbose: bool,
}

impl GlobalArgs {
    pub fn to_config(&self) -> Result<Config> {
        let config: Config = self.clone().try_into()?;
        Ok(config)
    }
}

impl TryInto<Config> for GlobalArgs  {
    type Error = Error;
    fn try_into(self) -> Result<Config> {
        fn get_current_git_dir() -> Result<PathBuf> {
            let git_dir: PathBuf = Repository::discover(env::current_dir()?)?.workdir().unwrap().to_path_buf();
            Ok(git_dir)
        }
        let mut config: Config = match (&self.config_path, &self.journal_dir) {
            (Some(c), Some(j)) => {
                Config::from_path_and_journal_dir(&c, &j)?
            },
            (Some(c), None) => {
                Config::try_from(c)?
            },
            (None, x) => {
                let result = match x {
                    Some(j) => {
                        Config::from_journal_dir(&j)
                    },
                    None => {
                        Config::from_journal_dir(&get_current_git_dir()?)
                    }
                };
                match result {
                    Ok(x) => x,
                    Err(_) => Config::default(),
                }
            },
        };
        if let Some(x) = &self.journal_dir {
            config.core.data_dir = x.clone();
        }
        Ok(config)
    }
}