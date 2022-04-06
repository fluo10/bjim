mod front_matter;
mod content;
mod bullet;
//mod section;
mod tag;

//mod task;

use std::convert::AsRef;
use std::path::{Path, PathBuf};
use std::fs::{File, OpenOptions, create_dir_all};
use std::io::prelude::*;
use std::io::{Write, BufWriter};
use regex::Regex;
use chrono::Local;

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
            let dir: &Path = self.path.parent().unwrap();
            if !dir.exists() {
                create_dir_all(dir);
            }
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
        println!("Migrating {:?} to {:?}", &src.path, &dst.path);
        let src_content: &mut PageContent = src.content.as_mut().unwrap();
        match (&src.front_matter, &dst.front_matter) {
            (Some(_x), None) => {
                dst.front_matter = src.front_matter.clone();
                dst.front_matter.as_mut().unwrap().update_date(Local::today().naive_local());
            },
            _ => {},
        };
        let mut dst_content: PageContent = PageContent::from_str(src_content.raw.as_str());
        
        src_content.replace_task_status(TaskStatus::Open,TaskStatus::Migrated);
        dst_content.replace_task_status(TaskStatus::Migrated, TaskStatus::Open);
        dst_content.filter_open_tasks();
        dst_content.replace_task_status(TaskStatus::Closed, TaskStatus::Open);
        dst.content = Some(dst_content);
    }

    fn split_content(&mut self) {
        let re:Regex = Regex::new(
            r"^(?:(?P<f>(?s)---\r?\n.*?(?-s))---\r?\n)?(?P<c>(?s).*(?-s))$"
        ).unwrap();
        let caps = re.captures(self.raw_content.as_str()).unwrap();
        self.front_matter = match caps.name("f") {
            Some(x) => Some(FrontMatter::from(x.as_str())),
            None => None,
        };
        self.content = Some(PageContent::from(caps.name("c").unwrap().as_str()));
    }
    fn join_content(&mut self) {
        self.raw_content = match &self.front_matter {
            Some(x) => x.to_string().unwrap() + "---\n" + self.content.as_ref().unwrap().raw.as_str(),
            None => String::from(self.content.as_ref().unwrap().raw.as_str()),
        };
        
    }

}


#[cfg(test)]
mod tests {
    use super::*;
    


    #[test]
    fn join_content() {
        fn assert_split(origin: &str, front_matter: &str, content: &str) {
            let mut page = Page { 
                path: PathBuf::new(),
                raw_content: String::from(origin),
                has_open_task: false,
                front_matter: None,
                content: None,
            };
            page.split_content();
            assert_eq!( match &page.front_matter {
                Some(x) => x.raw.as_str().clone(),
                None => "",
            }, front_matter);
            assert_eq!(page.content.unwrap().raw.as_str(), content);

            let mut page = Page{
                path: PathBuf::new(),
                raw_content: String::new(),
                has_open_task: false,
                front_matter: match front_matter {
                    "" => None, 
                    _ => Some(FrontMatter::from(front_matter))
                },
                content: Some(PageContent::from(content)),
            };
            page.join_content();
            assert_eq!(page.raw_content.as_str(), origin);
        }
        assert_split(
r"---
title: test
---
test
content
",
r"title: test
",
r"test
content
");
        assert_split(
r"---
title: test
--
test
content
",
r"",
r"---
title: test
--
test
content
");
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
