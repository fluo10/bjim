use crate::args::WriteArgs;

use clap::Parser;

/// Subcommand for update journal
#[derive(Parser)]
pub struct UpdateCmd {

    #[clap(flatten)]
    pub write_args: WriteArgs,

    #[clap(long)]
    pub push: bool,

    #[clap(long)]
    pub pull: bool,

}

impl UpdateCmd {
    pub fn run(&self) {
        
        // pull remote origin
        todo!();

        // Add daily log for today if not exist yet
        todo!();

        // Update link for access dailylog if needed
        todo!();

        // push remote origin
        todo!();

    }
}