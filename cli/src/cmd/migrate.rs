pub use clap::Parser;
pub use std::path::PathBuf;
use std::fs;
use crate::args::GlobalArgs;
use lib::Page;

#[derive(Parser)]
pub struct MigrateCmd {
    #[clap(short='n', long)]
    pub dry_run: bool,
    #[clap(flatten)]
    global: GlobalArgs,
    #[clap(name="source", value_name="FILE")]
    pub src_path: PathBuf,
    #[clap(name="Destination", value_name="FILE")]
    pub dst_path: PathBuf,

}

impl MigrateCmd {
    pub fn run(&self) {
        println!("Execute migrate");
        let mut src_pages: Vec<Page> = Vec::new();
        let mut dst_pages: Vec<Page> = Vec::new();
        
        if self.src_path.is_file() {
           src_pages.push(Page::new(&self.src_path));
        } else if self.src_path.is_dir() {
            for entry in fs::read_dir(self.src_path.as_path()).unwrap() {
                let entry = entry.unwrap();
                let path = entry.path();
                if path.is_file() {
                    src_pages.push(Page::new(&self.src_path));
                }
            }
        } else {
            panic!();
        }
        if self.dst_path.is_dir(){
            for page in src_pages.iter() {
                let filename = page.path.file_name().unwrap();
                let mut path = self.dst_path.clone();
                path.push(filename);
                dst_pages.push(Page::new(path));
            }
        } else if self.dst_path.is_file() & (src_pages.len() == 1) {
            dst_pages.push(Page::new(&self.dst_path));
        } else {
            panic!();
        }
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