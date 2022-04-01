use crate::args::GlobalArgs;

use lib::{Journal};

use anyhow::{Result};
use clap::Parser;




/// Migrate files based on collections
#[derive(Parser)]
pub struct CollectionMigrateCmd {
    #[clap(short='n', long)]
    dry_run: bool,
    #[clap(flatten)]
    global: GlobalArgs,
    collections: Vec<String>,
}

impl CollectionMigrateCmd {
    pub fn run(&self) -> Result<()> {
        
        match self.global.to_config().unwrap().globalize(){
            Ok(()) => {},
            Err(x) => {
                eprintln!("{}",x);
            }
        }

        let mut journal: Journal = Journal::new().unwrap();
        journal.reload();
        if self.collections.is_empty() {
            journal.migrate_collections()?;
        } else {
            for name in &self.collections {
                journal.migrate_collection(&name)?;
            }
        }
        Ok(())
    }
}