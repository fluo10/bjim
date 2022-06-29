use crate::{
    errors::ParseError,
    token::{Token, TokenKind},
    elements::Inline,
};


use std::collections::VecDeque;
use std::convert::{From, TryFrom};

use anyhow::anyhow;

type Result<T> = std::result::Result<T, ParseError>;

#[derive(Debug, PartialEq)]
pub struct CheckBox {
    pub open: Token,
    pub status: Token,
    pub close: Token,
    pub space: Token,
}

impl TryFrom<(Token, Token, Token, Token)> for CheckBox {
    type Error = ParseError;
    fn try_from(t: (Token, Token, Token, Token)) -> Result<CheckBox> {
        let (open, status, close, space) = t;
        if open.is_left_bracket() && status.len() == 1 && 
        close.is_right_bracket() && space.is_space() {
            Ok(
                CheckBox {
                    open: open,
                    status: status,
                    close: close,
                    space: space,
                }
            )
        } else {
            Err(ParseError::InvalidToken)
        }

    }
}

impl TryFrom<&mut VecDeque<Token>> for CheckBox {
    type Error = ParseError;
    fn try_from(t: &mut VecDeque<Token>) -> Result<Self> {
        match (t.get(0), t.get(1), t.get(2), t.get(3)) {
            (None, _, _, _) => {
                Err(ParseError::TokenNotFound)
            },
            (Some(open), Some(status), Some(close), Some(space)) 
                if open.is_left_bracket() && status.len() == 1 && 
                close.is_right_bracket() && space.is_space() => {
                    CheckBox::try_from((
                        t.pop_front().unwrap(),
                        t.pop_front().unwrap(),
                        t.pop_front().unwrap(),
                        t.pop_front().unwrap(),
                    ))
            },
            _ => Err(ParseError::InvalidToken)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn from_4_tokens() {
        let open = Token::from((TokenKind::LBracket, "["));
        let status = Token::from((TokenKind::Space, " "));
        let close = Token::from((TokenKind::RBracket, "]"));
        let space = Token::from((TokenKind::Space, " "));
        let from_tokens = CheckBox::try_from((
            open.clone(),
            status.clone(),
            close.clone(),
            space.clone(),
        )).unwrap();
        let expected = CheckBox{
            open: open,
            status: status,
            close: close,
            space: space,
        };
        assert_eq!(from_tokens, expected);
    }
}