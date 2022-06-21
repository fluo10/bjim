use super::page::Page;
use crate::Config;
use std::convert::AsRef;
use std::path::{Path};
use log::{info, debug, warn,};

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
        match self.migrate_collections(){
            Ok(_x) => {
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

    /// Migrate all collections in config automatically
    pub fn migrate_collections(&mut self) -> Result<()> {
        for (name, config) in &Config::global().collections {
            if config.auto_migration {
                self.migrate_collection(&name)?;
            }
        }
        Ok(())
    }

    /// Migrate template automatically based on config
    pub fn migrate_collection(&mut self, name: &str) -> Result<()> {
        let pages: Vec<&Path> = self.pages.iter().map(|p| p.path.as_path()).collect();
        debug!("Migrating: {}", name);
        let collection = &Config::global().collections.get(name).ok_or(anyhow!("Template {} is nothing in configure", name))?;
        match collection.migrate(Config::global(), &pages[..]) {
            Ok(_x) => {
                info!("Done migration: {}", name);
            },
            Err(_x) => {
                info!("Skip Migration: {}", name);
            }
        };
        Ok(())
    }
}

