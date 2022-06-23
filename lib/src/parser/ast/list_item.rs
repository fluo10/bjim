use super::Inline;
use crate::parser::Token;

#[derive(Debug, PartialEq)]
pub struct ListItemPrefix {
    pub indent: Option<Token>,
    pub bullet: Token,
    pub space: Token,
}

#[derive(Debug, PartialEq)]
pub struct Checkbox {
    pub open: Token,
    pub status: Token,
    pub close: Token,
    pub space: Token,
}

#[derive(Debug, PartialEq)]
pub struct ListItem {
    pub prefix: ListItemPrefix,
    pub checkbox: Option<Checkbox>,
    pub content: Vec<Inline>,
    pub children: Vec<ListItem>,
}