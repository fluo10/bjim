use crate::{
    errors::ParseError, 
    token::{Token, TokenKind},
    elements::Block,
};

use std::collections::VecDeque;
use std::convert::TryFrom;

type Result<T> = std::result::Result<T, ParseError>;

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