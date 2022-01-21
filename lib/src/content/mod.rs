use super::page::Page;
use std::path::{Path, PathBuf};
//use super::page::Page;
use walkdir::WalkDir;
use std::collections::HashMap;

pub struct Content{
    pub content_dir: PathBuf,
    pub pages: HashMap<PathBuf, Page>,
}

impl Content {
    pub fn new(path: &Path) -> Result<Self, String> {
        let mut content = Self {
            content_dir: path.to_path_buf(),
            pages: HashMap::with_capacity(4),
        };
        content.reload();
        Ok(content)
    }
    pub fn reload(&mut self) {
        for entry in walkdir::WalkDir::new(".").into_iter().filter_map(|e| e.ok()) {
            //pages.push(Page.from_path(file));
            self.pages.insert( entry.path().to_path_buf(), Page::new(entry.path()));
        }
    }
}