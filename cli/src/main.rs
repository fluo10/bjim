mod cli;
mod cmd;
use cli::Args;
mod args;
use clap::Parser;

fn main() {
    let args = Args::parse();
    args.command.run();
}