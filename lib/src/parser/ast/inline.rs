mod text;

pub use text::Text;

use crate::parser::{
    errors::ParseError,
    token::{Token, TokenKind},
};
use std::collections::VecDeque;
use std::convert::{From, TryFrom};

type Result<T> = std::result::Result<T, ParseError>;

#[derive(Debug, PartialEq)]
pub enum Inline {
    Text(Text),
    LineBreak(Token),
    //Bold,
    //Italic,
    //Link(),
    //HashTag(),
}

impl Inline {
    pub fn is_line_break(&self) -> bool {
        match self {
            Inline::LineBreak(_) => true,
            _ => false,
        }
    }
    pub fn is_text(&self) -> bool {
        match self {
            Inline::Text(_) => true,
            _ => false,
        }
    }
    
}

impl From<Text> for Inline {
    fn from(t: Text) -> Self {
        Inline::Text(t)
    }
}

/// Create inline from single token
impl TryFrom<Token> for Inline {
    type Error = ParseError;
    fn try_from(t: Token) -> Result<Self> {
        match &t.kind {
            TokenKind::LineBreak => {
                Ok(Inline::LineBreak(t))
            },
            _ => {
                Err(ParseError::InvalidToken)
            }
        }
    }
}

impl TryFrom<&mut VecDeque<Token>> for Inline {
    type Error = ParseError;
    fn try_from(t: &mut VecDeque<Token>) -> Result<Self> {
        match &t.front().ok_or(ParseError::TokenNotFound)?.kind {
            &TokenKind::LineBreak => {
                Ok(Inline::LineBreak(t.pop_front().unwrap()))
            },
            &TokenKind::LBracket => {
                todo!();
            },
            &TokenKind::HashTag => {
                todo!();
            },
            _ => {
                Ok(Text::try_from(t)?.into())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn parse_inline () {
        let mut tokens = VecDeque::from([
            Token::from((TokenKind::Text, "Test")),
            Token::from((TokenKind::LineBreak, "\n")),
            Token::from((TokenKind::Text, "NewLine")),
        ]);
        let mut inlines: Vec<Inline> = Vec::new();
        while let Ok(x) = Inline::try_from(&mut tokens) {
            inlines.push(x);
        }

        assert_eq!(inlines, vec![
            Inline::Text(Text::try_from(&mut VecDeque::from([Token::from((TokenKind::Text, "Test"))]).into()).unwrap()),
            Inline::LineBreak(Token::from((TokenKind::LineBreak, "\n"))),
            Inline::Text(Text::try_from(&mut VecDeque::from([Token::from((TokenKind::Text, "NewLine"))]).into()).unwrap()),
        ]);
    }

}