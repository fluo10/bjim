//mod front_matter;
//mod task;

use std::path::{Path, PathBuf};
//use front_matter::FrontMatter;
//use task::Task;

pub struct Page {
    path: PathBuf,
//    front_matter: FrontMatter;
//    content: Content;
}

impl Page {
    pub fn new(path: &Path) -> Self {
        Page { path: path.to_path_buf(),}
    }
    pub fn write(&self) {
        todo!()
    }
    pub fn migrate(&self, path: &Path) {
        todo!()
    }
}