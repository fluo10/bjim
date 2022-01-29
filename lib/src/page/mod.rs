//mod front_matter;
//mod task;
use std::env;
use std::convert::AsRef;
use std::path::{Path, PathBuf};
use std::fs::File;
use std::io::prelude::*;
use std::io::{BufRead, BufReader};
use regex::Regex;

//use front_matter::FrontMatter;
//use task::Task;

pub struct Page {
    pub path: PathBuf,
    pub raw_content: String,
    pub has_open_task: bool,
    pub bullets: Vec<Bullet>
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
        let reader = BufReader::new(f);
        for line in reader.lines() {
            self.raw_content.
        }
        f.read_to_string(&mut self.raw_content)
            .expect("something went wrong reading the file");
        let v: Vec<&str> = self.raw_content.as_str().matches("- [ ] ").collect();

        self.has_open_task = v.len() > 0;
        
    }
    pub fn write(&self) {
        todo!()
    }
    pub fn migrate_to(&mut self, page: &mut Self) {
        Self::migrate(self, page);
    }
    fn migrate(src: &mut Page, dst: &mut Self) {
        
        let re = Regex::new(r);
        dst.raw_content = src.raw_content.clone();
        src.open_to_migrated();
        dst.migrated_to_open();
        dst.extract_open_tasks();
    }

    fn extract_open_tasks(&mut self) {
        const REMOVE_TASK_PATTERN = r"^ *- +\[[^ ]\] .*$";
        const NOTE_PATTERN = r"^ *- +[^\[].*$";
    }

    fn open_to_migrated(&mut self) {
        todo!();
    }
    fn migrated_to_open(&mut self) {
        todo!();
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