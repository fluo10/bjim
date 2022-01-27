//mod front_matter;
//mod task;
use std::env;
use std::convert::AsRef;
use std::path::{Path, PathBuf};
use std::fs::File;
use std::io::prelude::*;
//use front_matter::FrontMatter;
//use task::Task;

pub struct Page {
    pub path: PathBuf,
    pub raw_content: String,
    pub has_open_task: bool,
//    front_matter: FrontMatter;
//    content: Content;
}

impl Page {
    pub fn new(path: impl AsRef<Path>) -> Self {
        Page { 
            path: path.as_ref().to_path_buf(),
            raw_content: String::new(),
            has_open_task: false,
        }
    }
    pub fn read(&mut self) {
        let mut f = File::open(self.path.as_path()).expect("file not found");
        f.read_to_string(&mut self.raw_content)
            .expect("something went wrong reading the file");
        let v: Vec<&str> = self.raw_content.as_str().matches("- [ ] ").collect();

        self.has_open_task = v.len() > 0;
        
    }
    pub fn write(&self) {
        todo!()
    }
    pub fn migrate(&self, path: &Path) {
        todo!()
    }
}
/*
#[cfg(test)]
mod tests {
    use super::*;
    [#test]
    fn test_io (){

        /*let content: String = r#######"
        ---
        title: "title"
        date: 2022-01-27
        categories: 
          - cat
        tags:
          - tail
        ---

        - [ ] Open task
        - [ ] Closed task

        "#######;
        */
    }
}
*/