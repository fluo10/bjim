mod front_matter;
mod content;
mod bullet;
//mod task;
use std::env;
use std::convert::AsRef;
use std::path::{Path, PathBuf};
use std::fs::{File, OpenOptions};
use std::io::prelude::*;
use std::io::{BufRead, BufReader, Write, BufWriter};
use regex::Regex;

use front_matter::FrontMatter;
use content::PageContent;
use bullet::TaskStatus;

pub struct Page {
    pub path: PathBuf,
    pub raw_content: String,
    pub has_open_task: bool,
//    pub bullets: Vec<Bullet>,
    front_matter: Option<FrontMatter>,
    pub content: Option<PageContent>,
}

impl Page {
    pub fn new(path: impl AsRef<Path>) -> Self {
        Page { 
            path: path.as_ref().to_path_buf(),
            raw_content: String::new(),
            has_open_task: false,
            front_matter: None,
            content: None,
        }
    }
    pub fn read(&mut self) {
        let mut f = File::open(self.path.as_path()).expect("file not found");
        /*
        let reader = BufReader::new(f);
        for line in reader.lines() {
            self.raw_content.
        }
        */
        f.read_to_string(&mut self.raw_content)
            .expect("something went wrong reading the file");
        let c = self.raw_content.clone();
        let v: Vec<&str> = c.as_str().matches("- [ ] ").collect();
        self.split_content();
        self.has_open_task = v.len() > 0;
        
    }
    pub fn write(&mut self) {
        let file;
        if self.path.is_file() {
            file= OpenOptions::new().write(true).open(self.path.clone()).unwrap();
        } else if !self.path.exists() {
            file = File::create(self.path.clone()).unwrap();
        } else {
            panic!();
        }
        let mut buf = BufWriter::new(file);
        self.join_content();
        buf.write(self.raw_content.as_bytes()).unwrap();
    }
    pub fn migrate_to(&mut self, page: &mut Self) {
        Self::migrate(self, page);
    }
    fn migrate(src: &mut Page, dst: &mut Self) {
        let mut src_content: &mut PageContent = src.content.as_mut().unwrap();
        match (&src.front_matter, &dst.front_matter) {
            (Some(x), None) => {
                dst.front_matter = src.front_matter.clone();
            },
            _ => {},
        };
        let mut dst_content: PageContent = PageContent::from_str(src_content.raw.as_str());
        
        src_content.replace_task_status(TaskStatus::Open,TaskStatus::Migrated);
        dst_content.replace_task_status(TaskStatus::Migrated, TaskStatus::Open);
        dst_content.filter_open_tasks();
        dst.content = Some(dst_content);
    }

    fn split_content(&mut self) {
        let re:Regex = Regex::new(
            r"^[[:space:]]*---(\r?\n(?s).*?(?-s))---[[:space:]]*(?:$|(?:\r?\n((?s).*(?-s))$))"
        ).unwrap();
        let caps = re.captures(self.raw_content.as_str()).unwrap();
        self.front_matter = Some(FrontMatter::from_str(caps.get(1).unwrap().as_str()));
        self.content = Some(PageContent::from_str(caps.get(2).unwrap().as_str()));
    }
    fn join_content(&mut self) {
        self.raw_content = match &self.front_matter {
            Some(x) => String::from("---") +  self.front_matter.as_ref().unwrap().raw.as_str() + "---\n" + self.content.as_ref().unwrap().raw.as_str(),
            None => String::from(self.content.as_ref().unwrap().raw.as_str()),
        };
        
    }

}

fn split_content(content: &str) -> Result<(FrontMatter,PageContent), std::io::Error> {
    
    let re:Regex = Regex::new(
        r"^[[:space:]]*---(\r?\n(?s).*?(?-s))---[[:space:]]*(?:$|(?:\r?\n((?s).*(?-s))$))"
    ).unwrap();
    let caps = re.captures(content).unwrap();
    let front_matter = FrontMatter::from_str(caps.get(1).unwrap().as_str());
    let content = PageContent::from_str(caps.get(2).unwrap().as_str());
    Ok((front_matter, content))
}


#[cfg(test)]
mod tests {
    use super::*;
    
    const RAW: &str =r"---
title: test
---
test
content
";
    const HEADER: &str =r"
title: test
";
    const CONTENT: &str = r"test
content
";

    #[test]
    fn join_content() {
        let mut page = Page { 
            path: PathBuf::new(),
            raw_content: String::from(RAW),
            has_open_task: false,
            front_matter: None,
            content: None,
        };
        page.split_content();
        assert_eq!(page.front_matter.unwrap().raw.as_str(), HEADER);
        assert_eq!(page.content.unwrap().raw.as_str(), CONTENT);

        let mut page = Page{
            path: PathBuf::new(),
            raw_content: String::new(),
            has_open_task: false,
            front_matter: Some(FrontMatter::from_str(HEADER)),
            content: Some(PageContent::from_str(CONTENT)),
        };
        page.join_content();
        assert_eq!(page.raw_content.as_str(), RAW);
        
    }

    #[test]
    fn split_content() {

    }
    
    /*
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
    }*/
}
