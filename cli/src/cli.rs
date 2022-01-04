use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[clap(about, version, author)]
pub struct Args {
    /// Name of the person to greet
    #[clap(short, long)]
    pub name: String,

    /// Number of times to greet
    #[clap(short, long, default_value_t = 1)]
    pub count: u8,
}

