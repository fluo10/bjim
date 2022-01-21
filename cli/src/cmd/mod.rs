//pub mod config;
pub mod show_tasks;
//pub mod migrate;
pub mod check;
pub use check::Check;

pub trait Command {
    fn run(&self);
}

pub trait Filter {

}