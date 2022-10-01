use super::page::Page;
use config::{Config, CoreConfig, CollectionConfig};
use std::convert::AsRef;
use std::fs;
use std::path::{Path};

use chrono::{Date, Local};
use log::{info, debug, warn,};

//use super::page::Page;


use anyhow::{anyhow, bail, Result};

pub struct Journal{
    pub config: Config,
    pub pages: Vec<Page>,
}

impl Journal {
    pub fn new() -> Result<Self> {
        let mut data = Self {
            config: Config::default(),
            pages: Vec::new(),
        };
        data.reload();
        Ok(data)
    }
    pub fn reload(&mut self) {
        let config = &self.config;
        for entry in walkdir::WalkDir::new(config.core.data_dir.clone()).into_iter().filter_map(|e| {
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
    pub fn migrate_collections_auto(&mut self, Vec<&str>) -> Result<()> {
        let pages: Vec<&Path> = self.pages.iter().map(|p| p.path.as_path()).collect();

        for (name, config) in &self.config.collections.iter().filter(|(k, v)| v.auto_migration) {
            if config.auto_migration {
                &*self.migrate_collection(&name)?;
            }
        }
        Ok(())
    }

    /// Migrate template automatically based on config
    pub fn migrate_collection(&mut self, col_name: Option<&str>) -> Result<()> {
        let pages: Vec<&Path> = self.pages.iter().map(|p| p.path.as_path()).collect();
        debug!("Migrating: {}", col_name );
        let collection = &self.config.collections.get(col_name).ok_or(anyhow!("Template {} is nothing in configure", col_name))?;
        match migrate_collection(&self.config, collection, &pages[..]) {
            Ok(_x) => {
                info!("Done migration: {}", col_name);
            },
            Err(_x) => {
                info!("Skip Migration: {}", col_name);
            }
        };
        Ok(())
    }
}

fn migrate_collection(config: &Config, col_config: &CollectionConfig, pages: &[&Path], ) -> Result<Page>{
    let core_config = &config.core;
    let tags_config = &config.tags;
    let date: Date<Local> = Local::today();
    if let Some((latest_path, period)) = col_config.get_latest_path_period(&pages) {
        if period.contains(date.naive_local()) {
            Err(anyhow::anyhow!("Migration is not needed"))
        } else {
            let mut latest_page = Page::new(&latest_path);
            let today_path = core_config.data_dir.join(col_config.get_path(date.naive_local()).unwrap());
            let mut today_page = Page::new(today_path);
            latest_page.read();
            latest_page.migrate_to(&mut today_page, Some(config));
            if !core_config.read_only {
                latest_page.write();
                today_page.write();
            }

            if let Some(archive) = &col_config.archive_path {
                let archive_path = archive.get_path(period.start);
                if archive_path != latest_path {
                    fs::rename(latest_path, archive_path)?;
                }
            }

            /*
            if let Some(x) = &self.link_path {
                let link_path = self.config.data_dir.join(x);
                trace!("Updating link {:?}", &link_path);
                match (link_path.is_symlink(), link_path.exists()) {
                    (true, _) => {
                        remove_file(&link_path);
                        symlink(&today_page.path, &link_path);
                        info!("Complete updating link {:?}", &link_path);
                    },
                    (false, true) => {
                        warn!("Skip updating link: {:?}", &link_path);
                    }
                    (false, false) => {
                        symlink(&today_page.path, &link_path);
                        info!("Create new link {:?}", &link_path);
                    }
                }
            }
            */

            Ok(today_page)
        }
    } else {
        bail!("Latest page is not found");
    }
        
}