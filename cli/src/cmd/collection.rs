mod migrate;

use migrate::CollectionMigrateCmd;



use anyhow::Result;
use clap::{Parser, Subcommand};




/// Subcommands for using Collection
#[derive(Parser)]
pub struct CollectionCmd {

    #[clap(subcommand)]
    pub command: CollectionCommands,
    
}

#[derive(Subcommand)]
pub enum CollectionCommands {
    Migrate(CollectionMigrateCmd),
}

impl CollectionCmd {
    pub fn run(&self) -> Result<()> {
        match &self.command {
            CollectionCommands::Migrate(x) => x.run(),
        };
        Ok(())
    }
}