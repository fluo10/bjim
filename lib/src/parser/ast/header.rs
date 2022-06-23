use super::Inline;
use crate::parser::Token;

#[derive(Debug, PartialEq)]
pub struct HeaderPrefix {
    pub prefix: Token,
    pub space: Token,
}
#[derive(Debug, PartialEq)]
pub struct Header {
    pub prefix: HeaderPrefix,
    pub content: Vec<Inline>
}