//! A module for searching files with filtering
use chrono::{Date, Local};
use clap::Args;

#[derive(Args)]
pub struct PageArgs {
    #[clap(long)]
    date: Option<String>,
    #[clap(long)]
    tags: Vec<String>,
    #[clap(long)]
    categories: Vec<String>,
    #[clap(long)]
    title: Option<String>,
}

