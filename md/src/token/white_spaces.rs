use super::TokenPosition;

use std::fmt;

#[derive(Clone, Debug, PartialEq)]
pub struct IndentToken{
    position: TokenPosition,
    literal: String,   
}

impl IndentToken {
    pub fn len(&self) -> usize {
        self.literal.len()
    }
}

impl fmt::Display for IndentToken{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.literal)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct SpaceToken{
    position: TokenPosition,
    literal: String,   
}

impl SpaceToken {
    pub fn len(&self) -> usize {
        self.literal.len()
    }
}

impl fmt::Display for SpaceToken{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.literal)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct LineBreakToken{
    position: TokenPosition,
    literal: String,   
}

impl LineBreakToken {
    pub fn len(&self) -> usize {
        self.literal.len()
    }
}

impl fmt::Display for LineBreakToken{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.literal)
    }
}