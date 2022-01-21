//mod front_matter;
//mod task;

use std::convert::AsRef;
use std::path::{Path, PathBuf};
//use front_matter::FrontMatter;
//use task::Task;

pub struct Page {
    pub path: PathBuf,
//    front_matter: FrontMatter;
//    content: Content;
}

impl Page {
    pub fn new(path: impl AsRef<Path>) -> Self {
        Page { path: path.as_ref().to_path_buf(),}
    }
    pub fn write(&self) {
        todo!()
    }
    pub fn migrate(&self, path: &Path) {
        todo!()
    }
}