use super::page::Page;
use crate::Config;
use std::convert::AsRef;
use std::path::{Path, PathBuf};
use log::{info, debug, trace, warn,};

//use super::page::Page;


use anyhow::{anyhow,Result};

pub struct Journal{
    pub pages: Vec<Page>,
}

impl Journal {
    pub fn new() -> Result<Self> {
        let mut data = Self {
            pages: Vec::new(),
        };
        data.reload();
        Ok(data)
    }
    pub fn reload(&mut self) {
        let config = Config::global();
        for entry in walkdir::WalkDir::new(config.data_dir.clone()).into_iter().filter_map(|e| {
            if e.as_ref().unwrap().path().extension()?.to_str().unwrap() == "md" {
                e.ok()
            } else {
                None
            }
        }) {
            //pages.push(Page.from_path(file));
            self.pages.push(Page::new(entry.path()));
        }
    }
    pub fn read(&mut self) {
        for page in &mut self.pages {
            page.read();
        }
    }
    pub fn update(&mut self) {
                
        // pull remote origin
        //todo!();

        // Add daily log for today if not exist yet
        println!("Migrating regular log");
        match self.migrate_template_all(){
            Ok(x) => {
                info!("done");
            }
            Err(e) => {
                warn!("{}", e);
            }
        }

        // Update link for access dailylog if needed
        //todo!();

        // push remote origin
        //todo!();

    }

    /// Migrate all templates in config automatically
    pub fn migrate_template_all(&mut self) -> Result<()> {
        for (name, template) in &Config::global().templates {
            self.migrate_template(&name)?;
        }
        Ok(())
    }

    /// Migrate template automatically based on config
    pub fn migrate_template(&mut self, name: &str) -> Result<()> {
        let pages: Vec<&Path> = self.pages.iter().map(|p| p.path.as_path()).collect();
        debug!("Migrating: {}", name);
        let template = &Config::global().templates.get(name).ok_or(anyhow!("Template {} is nothing in configure", name))?;
        match template.regular_migration(&pages[..]) {
            Ok(x) => {
                info!("Done migration: {}", name);
            },
            Err(x) => {
                info!("Skip Migration: {}", name);
            }
        };
        Ok(())
    }
}

