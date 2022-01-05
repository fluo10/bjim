//pub mod config;
pub mod show_tasks;
//pub mod migrate;

pub trait Command {
    fn run(&self);
}
