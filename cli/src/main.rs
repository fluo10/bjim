mod cli;
use cli::Args;
use clap::Parser;

fn main() {
    let args = Args::parse();
    match &args.command {
        cli::Commands::ShowTasks => {
            println!("Run showtasks");}
        cli::Commands::Migrate => {
            println!("Run migrate");
        }
    }
}