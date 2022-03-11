pub use clap::Parser;
pub use std::path::PathBuf;
use std::fs::{create_dir, read_dir};
use crate::args::GlobalArgs;
use lib::Page;

#[derive(Parser)]
pub struct MigrateCmd {
    #[clap(short='n', long)]
    pub dry_run: bool,
    #[clap(flatten)]
    global: GlobalArgs,
    #[clap(min_values=2, value_names=&["SOURCE", "DESTINATION"])]
    pub paths: Vec<PathBuf>,
}

impl MigrateCmd {
    pub fn run(&self) {
        println!("Execute migrate");
        let mut src_paths: Vec<PathBuf> = self.paths.clone();
        let dst_path: PathBuf = src_paths.pop().unwrap();
        if src_paths.len() > 2 {
            if dst_path.is_file() {
                println!("Destination is not dir!");
                ()
            } else if !dst_path.exists() {
                create_dir(&dst_path);
            }
            assert!(dst_path.is_dir())
        }

        let mut src_pages: Vec<Page> = Vec::new();
        let mut dst_pages: Vec<Page> = Vec::new();
        
        for path in src_paths {
            if path.is_file() {
                src_pages.push(Page::new(&path));
            } else if path.is_dir() {
                for entry in read_dir(path.as_path()).unwrap() {
                    let entry = entry.unwrap();
                    let path = entry.path();
                    if path.is_file() {
                        src_pages.push(Page::new(&path));
                    }
                }
            } else {
                panic!();
            }
        }
        if dst_path.is_dir(){
            for page in src_pages.iter() {
                let filename = page.path.file_name().unwrap();
                let mut path = dst_path.clone();
                path.push(filename);
                dst_pages.push(Page::new(path));
            }
        } else {
            dst_pages.push(Page::new(dst_path));
        }
        assert_eq!(src_pages.len(), dst_pages.len());
        for (mut src_page, mut dst_page) in src_pages.into_iter().zip(dst_pages.into_iter()) {
            src_page.read();
            src_page.migrate_to(&mut dst_page);
            if !self.dry_run {
                src_page.write();
                dst_page.write();
            }
        }
        //println!("from: {}", self.src_path);
        //println!("to:   {}", self.dst_path);
    }
}