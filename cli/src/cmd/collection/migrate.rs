use crate::args::GlobalArgs;

use bjim_lib::{Journal};

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
        
        let config = self.global.to_config()?;

        let mut journal: Journal = Journal::new().unwrap();
        journal.reload();
        if self.collections.is_empty() {
            journal.migrate_collections_auto()?;
        } else {
            for name in &self.collections {
                journal.migrate_collection(&name)?;
            }
        }
        Ok(())
    }
}