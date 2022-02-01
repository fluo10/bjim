use clap::Args;

/// Args for sub command writing file
#[derive(Args)]
pub struct WriteArgs {
    #[clap(short, long)]
    dry_run: bool,
    #[clap(short, long)]
    interactive: bool,
    #[clap(short, long)]
    force: bool,
}