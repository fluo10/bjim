use super::TokenPosition;

use std::fmt;

#[derive(Clone, Debug, PartialEq)]
pub struct CodeBlockFenceToken{
    position: TokenPosition,
    literal: String,   
}

impl CodeBlockFenceToken {
    pub fn len(&self) -> usize {
        self.literal.len()
    }
}

impl fmt::Display for CodeBlockFenceToken{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.literal)
    }
}