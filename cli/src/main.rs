mod cli;
mod cmd;
use cli::Args;
use clap::Parser;

fn main() {
    let args = Args::parse();
    args.command.run();
}