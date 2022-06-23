use crate::parser::Token;
use super::ListItem;

#[derive(Debug, PartialEq)]
pub struct List {
    pub content: Vec<ListItem>,
}