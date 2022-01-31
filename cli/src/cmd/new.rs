pub use clap::Parser;

#[derive(Parser)]
pub struct AddCmd {
    #[clap(short="-n", long)]
    pub dry_run: bool,
    pub source_path: String,
    pub destination_path: String,

}

impl AddCmd {
    pub fn run(&self) {
        println!("Execute migrate");
        
        println!("from: {}", self.destination_path);
        println!("to:   {}", self.destination_path);
    }
}