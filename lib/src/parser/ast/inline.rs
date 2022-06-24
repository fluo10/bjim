use crate::parser::Token;
use super::Text;
use std::convert::From;

#[derive(Debug, PartialEq)]
pub enum Inline {
    Text(Text),
    LineBreak(Token),
}

impl From<Text> for Inline {
    fn from(t: Text) -> Self {
        Inline::Text(t)
    }
}
