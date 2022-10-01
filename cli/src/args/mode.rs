use clap::Args;

use bjim_config::Config;

/// Args for sub command writing file
#[derive(Args)]
pub struct ModeArgs {
    #[clap(short, long)]
    dry_run: bool,
    // #[clap(short, long)]
    // interactive: bool,
    // #[clap(short, long)]
    // force: bool,
}

impl ModeArgs {
    pub fn set_config (&self, config: &mut Config){
        config.core.read_only = self.dry_run;

    }
}