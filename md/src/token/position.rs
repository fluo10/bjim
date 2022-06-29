use std::convert::{From, Into};

use std::fmt;

#[derive(Clone, Copy, Debug, PartialEq,)]
pub struct TokenPosition{
    pub line: usize, 
    pub column: usize,
}

impl From<(usize, usize)> for TokenPosition{
    fn from(f: (usize, usize)) -> Self {
        TokenPosition{
            line: f.0,
            column: f.1,
        }
    }
}

impl From<TokenPosition> for (usize, usize) {
    fn from(t: TokenPosition) -> (usize, usize) {
        (t.line, t.column)
    }
}
