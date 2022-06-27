use super::Inline;
use crate::parser::Token;

use std::collections::VecDeque;
use std::convert::From;

#[derive(Debug, PartialEq)]
pub struct HeaderPrefix {
    pub prefix: Token,
    pub space: Token,
}

impl From<&mut VecDeque<Token>> for HeaderPrefix {
    fn from(t:&mut VecDeque<Token>) -> Self {
        let prefix= t.pop_front().unwrap();
        let space= t.pop_front().unwrap();
        HeaderPrefix{
            prefix: prefix,
            space: space,
        }
    }
    
}
#[derive(Debug, PartialEq)]
pub struct Header {
    pub prefix: HeaderPrefix,
    pub content: Vec<Inline>
}
