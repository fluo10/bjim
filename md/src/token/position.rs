use crate::token::{LexedToken, TokenLike};

use std::convert::{From, Into};
use std::ops::{Add, AddAssign};

use std::fmt;

#[derive(Clone, Copy, Debug, PartialEq,)]
pub struct TokenPosition{
    pub line: usize, 
    pub column: usize,
}

impl TokenPosition{
    pub fn new() -> TokenPosition {
        Self{
            line: 1,
            column: 1,
        }
    }
}

/// Examples
/// 
/// ```
/// use bjim_md::lexer::LexedToken;
/// use bjim_md::token::*;
/// use std::ops::Add;
/// let position = TokenPosition::new();
/// 
/// let token = LexedToken::from(WordToken::try_from("word").unwrap());
/// assert_eq!((TokenPosition::new() + &token), TokenPosition::from((1,5)));
/// 
/// let line_break_token = LexedToken::from(LineBreakToken::try_from("\n").unwrap());
/// assert_eq!((TokenPosition::new() + &line_break_token), TokenPosition::from((2,1)));
impl Add<& LexedToken> for TokenPosition {
    type Output = TokenPosition;
    fn add(mut self, token: &LexedToken) -> TokenPosition {
        match token {
            LexedToken::LineBreak(_) => {
                self.line += 1;
                self.column = 1;
            },
            _ => {
                self.column += token.len();
            }
        }
        self
        
    }
}

impl AddAssign<& LexedToken> for TokenPosition {
    fn add_assign(&mut self, token: &LexedToken) {
        match token {
            LexedToken::LineBreak(_) => {
                self.line += 1;
                self.column = 1;
            },
            _ => {
                self.column += token.len();
            }
        }
    }
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
