use crate::{
    errors::ParseError, 
    token::{Token, TokenKind},
};

use std::collections::VecDeque;
use std::convert::TryFrom;

type Result<T> = std::result::Result<T, ParseError>;
#[derive(Debug, PartialEq)]
pub struct Text {
    pub content: Vec<Token>,
}

impl Text {
    pub fn new() -> Text {
        Text{
            content: Vec::new()
        }
    }
}


impl TryFrom<&mut VecDeque<Token>> for Text {
    type Error = ParseError;
    fn try_from(t: &mut VecDeque<Token>) -> Result<Self> {
        let mut text = Text::new();
        while let Some(x) = t.front() {
            match &x.kind {
                &TokenKind::LineBreak => {
                    break;
                },
                _ => {
                    text.content.push(t.pop_front().unwrap());
                }
            }
        }
        Ok(text)
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn parse_text() {
        use super::*;
        let mut tokens= VecDeque::from(vec![
            Token::from((TokenKind::Text, "test")),
                Token::from((TokenKind::Space, " ")),
                Token::from((TokenKind::Text, "text")),
                Token::from((TokenKind::LineBreak, "\n")),
        ]);
        assert_eq!(Text::try_from(&mut tokens).unwrap(), Text{
            content: vec![
                Token::from((TokenKind::Text, "test")),
                Token::from((TokenKind::Space, " ")),
                Token::from((TokenKind::Text, "text")),
            ]
        });
    }
}