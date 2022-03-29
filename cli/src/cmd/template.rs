mod migrate;

use migrate::TemplateMigrateCmd;
use crate::args::GlobalArgs;
use lib::Page;

use anyhow::Result;
use clap::{Parser, Subcommand};

use std::fs::{create_dir, read_dir};
use std::path::PathBuf;

/// Subcommands for using template
#[derive(Parser)]
pub struct TemplateCmd {

    #[clap(subcommand)]
    pub command: TemplateCommands,
    
}

#[derive(Subcommand)]
pub enum TemplateCommands {
    Migrate(TemplateMigrateCmd),
}

impl TemplateCmd {
    pub fn run(&self) -> Result<()> {
        match &self.command {
            TemplateCommands::Migrate(x) => x.run(),
        };
        Ok(())
    }
}