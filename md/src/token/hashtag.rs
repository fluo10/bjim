use super::TokenPosition;

use std::fmt;

#[derive(Clone, Debug, PartialEq)]
pub struct HashtagToken{
    position: TokenPosition,
    literal: String,   
}

impl HashtagToken {
    pub fn len(&self) -> usize {
        self.literal.len()
    }
}

impl fmt::Display for HashtagToken{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.literal)
    }
}