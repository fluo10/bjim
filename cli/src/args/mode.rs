use clap::Args;

use lib::Config;

/// Args for sub command writing file
#[derive(Args)]
pub struct ModeArgs {
    #[clap(short, long)]
    dry_run: bool,
    #[clap(short, long)]
    interactive: bool,
    #[clap(short, long)]
    force: bool,
}

impl ModeArgs {
    pub fn add_config (&self, config: &mut Config){
        config.dry_run = self.dry_run;

    }
}