use super::TokenPosition;

use std::fmt;

#[derive(Clone, Debug, PartialEq)]
pub struct WordToken{
    position: TokenPosition,
    literal: String,   
}

impl WordToken {
    pub fn len(&self) -> usize {
        self.literal.len()
    }
}

impl fmt::Display for WordToken{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.literal)
    }
}