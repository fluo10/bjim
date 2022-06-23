use crate::parser::Token;
use super::Inline;

#[derive(Debug, PartialEq)]
pub struct Paragraph {
    pub content: Vec<Inline>,
}