use crate::parser::Token;
use super::ListItem;

#[derive(Debug, PartialEq)]
pub struct List {
    pub content: Vec<ListItem>,
}

impl List {
    pub fn new() -> Self {
        List {
            content: Vec::new(),
        }
    }
}