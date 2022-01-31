pub use clap::Parser;
pub use std::path::PathBuf;
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
        let mut src_page: Page;
        let mut dst_page: Page;
        
        if self.src_path.is_file() {
           src_page = Page::new(&self.src_path);
        } else {
            panic!();
        }
        if !self.dst_path.is_dir(){
            dst_page = Page::new(&self.dst_path);
        } else {
            panic!();
        }
        src_page.migrate_to(&mut dst_page);
        if !self.dry_run {
            src_page.write();
            dst_page.write();
        }
        //println!("from: {}", self.src_path);
        //println!("to:   {}", self.dst_path);
    }
}