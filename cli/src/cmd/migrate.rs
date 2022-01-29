pub use clap::Parser;

#[derive(Parser)]
pub struct MigrateCmd {
    #[clap(short, long)]
    pub dry_run: bool,
    pub source_path: Option<String>,
    pub destination_path: String,

}

impl MigrateCmd {
    pub fn run(&self) {
        println!("Execute migrate");
        match &self.source_path {
            Some(x) => {
                println!("from: {}", x);
            }
            None => {}
        }
        println!("to:   {}", self.destination_path);
    }
}