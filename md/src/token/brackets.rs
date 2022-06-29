use super::TokenPosition;

use std::fmt;

#[derive(Clone, Debug, PartialEq)]
pub struct LeftBracketToken{
    position: TokenPosition,
    literal: String,   
}

impl fmt::Display for LeftBracketToken{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.literal)
    }
}

impl LeftBracketToken {
    pub fn len(&self) -> usize {
        self.literal.len()
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct RightBracketToken{
    position: TokenPosition,
    literal: String,   
}

impl RightBracketToken {
    pub fn len(&self) -> usize {
        self.literal.len()
    }
}

impl fmt::Display for RightBracketToken{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.literal)
    }
}