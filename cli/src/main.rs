mod cmd;
mod args;
mod errors;

use cmd::Cmd;

use env_logger::{Builder, Env};
use clap::Parser;

fn main() {
    Builder::from_env(Env::default().default_filter_or("info"))
        .format_timestamp(None)
        .init();
    let cmd = Cmd::parse();
    cmd.run();
}