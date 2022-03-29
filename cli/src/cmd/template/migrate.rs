use crate::args::GlobalArgs;

use lib::Page;

use anyhow::{Result};
use clap::Parser;

use std::fs::{create_dir, read_dir};
use std::path::PathBuf;

/// Migrate files based on templates
#[derive(Parser)]
pub struct TemplateMigrateCmd {
    #[clap(short='n', long)]
    dry_run: bool,
    #[clap(flatten)]
    global: GlobalArgs,
    templates: Vec<String>,
}

impl TemplateMigrateCmd {
    pub fn run(&self) -> Result<()> {
        todo!();
    }
}