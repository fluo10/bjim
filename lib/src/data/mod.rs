use super::page::Page;
use std::path::{Path, PathBuf};
//use super::page::Page;
use walkdir::WalkDir;
use std::collections::HashMap;

pub struct Data{
    pub data_dir: PathBuf,
    pub pages: HashMap<PathBuf, Page>,
}

impl Data {
    pub fn new(path: &Path) -> Result<Self, String> {
        let mut content = Self {
            data_dir: path.to_path_buf(),
            pages: HashMap::with_capacity(4),
        };
        content.reload();
        Ok(content)
    }
    pub fn reload(&mut self) {
        for entry in walkdir::WalkDir::new(".").into_iter().filter_map(|e| {
            if e.as_ref().unwrap().path().extension()?.to_str().unwrap() == "md" {
                e.ok()
            } else {
                None
            }
        }) {
            //pages.push(Page.from_path(file));
            self.pages.insert( entry.path().to_path_buf(), Page::new(entry.path()));
        }
    }
}

