use super::Block;
use crate::parser::Token;

#[derive(Debug, PartialEq)]
pub struct Body {
    pub content: Vec<Block>,
}