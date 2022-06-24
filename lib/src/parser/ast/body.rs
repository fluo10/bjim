use super::Block;
use crate::parser::Token;

#[derive(Debug, PartialEq)]
pub struct Body {
    pub content: Vec<Block>,
}

impl Body {
    pub fn new() -> Body {
        Body{
            content: Vec::new(),
        }
    }
}