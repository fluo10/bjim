mod cmd;
mod args;
use cmd::Cmd;

use clap::Parser;

fn main() {
    let cmd = Cmd::parse();
    cmd.run();
}