//! A module for searching files with filtering

use clap::Args;

/// Args for specify page meta data used to search or create page
#[derive(Args, Debug, Default, PartialEq)]
pub struct PageArgs {
    #[clap(long)]
    date: Option<String>,
    #[clap(long, multiple_occurrences = true)]
    tags: Vec<String>,
    #[clap(long, multiple_occurrences = true)]
    categories: Vec<String>,
    #[clap(long)]
    title: Option<String>,
    #[clap(long)]
    collection: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::OsString;
    use clap::Parser;

    #[derive(Parser, Debug, Default, PartialEq)]
    struct PageArgsParser {
        #[clap(flatten)]
        page_args: PageArgs,
    }
    
    #[test]
    fn parse() {
        let args: Vec<OsString> = Vec::new();
        assert_eq!(
            PageArgsParser::parse_from(&args),
            PageArgsParser::default()
        );
    }
}