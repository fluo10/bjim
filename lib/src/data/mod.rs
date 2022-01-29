use super::page::Page;
use std::convert::AsRef;
use std::path::{Path, PathBuf};
//use super::page::Page;
use walkdir::WalkDir;
use std::collections::HashMap;

pub struct Data{
    pub data_dir: PathBuf,
    pub pages: Vec<Page>,
}

impl Data {
    pub fn new(path: impl AsRef<Path>) -> Result<Self, String> {
        let mut data = Self {
            data_dir: path.as_ref().to_path_buf(),
            pages: Vec::new(),
        };
        data.reload();
        Ok(data)
    }
    pub fn reload(&mut self) {
        for entry in walkdir::WalkDir::new(self.data_dir.to_path_buf()).into_iter().filter_map(|e| {
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
}

