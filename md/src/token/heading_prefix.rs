use super::TokenPosition;

use std::fmt;

#[derive(Clone, Debug, PartialEq)]
pub struct HeadingPrefixToken{
    position: TokenPosition,
    literal: String,   
}

impl HeadingPrefixToken {
    pub fn len(&self) -> usize {
        self.literal.len()
    }
}

impl fmt::Display for HeadingPrefixToken{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.literal)
    }
}