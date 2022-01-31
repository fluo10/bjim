pub use clap::Parser;
pub use std::path::PathBuf;

#[derive(Parser)]
pub struct MigrateCmd {
    #[clap(short='n', long)]
    pub dry_run: bool,
    #[clap(short, long)]
    pub verbose: bool,
    #[clap(short, long)]
    pub config: Option<PathBuf>,
    pub source: String,
    pub destination: String,

}

impl MigrateCmd {
    pub fn run(&self) {
        println!("Execute migrate");
        
        println!("from: {}", self.source);
        println!("to:   {}", self.destination);
    }
}