use super::page::Page;
use crate::Config;
use std::convert::AsRef;
use std::path::{Path, PathBuf};
//use super::page::Page;
use walkdir::WalkDir;
use std::collections::HashMap;
use anyhow::Result;

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
        for mut page in &mut self.pages {
            page.read();
        }
    }
    pub fn update(&mut self) {
                
        // pull remote origin
        todo!();

        // Add daily log for today if not exist yet
        todo!();

        // Update link for access dailylog if needed
        todo!();

        // push remote origin
        todo!();

    }
}

