use crate::args::GlobalArgs;

use lib::{Page, Journal};

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
        
        match self.global.to_config().unwrap().globalize(){
            Ok(()) => {},
            Err(x) => {
                eprintln!("{}",x);
            }
        }

        let mut journal: Journal = Journal::new().unwrap();
        journal.reload();
        if self.templates.is_empty() {
            journal.migrate_template_all()?;
        } else {
            for name in &self.templates {
                journal.migrate_template(&name)?;
            }
        }
        Ok(())
    }
}