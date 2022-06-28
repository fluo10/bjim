use crate::{
    errors::ParseError, 
    token::{Token, TokenKind},
    elements::Inline,
};

use std::collections::VecDeque;
use std::convert::TryFrom;

type Result<T> = std::result::Result<T, ParseError>;

#[derive(Debug, PartialEq)]
pub struct Paragraph {
    pub content: Vec<Inline>,
}

impl From<Vec<Inline>> for Paragraph {
    fn from(t: Vec<Inline>) -> Self {
        Paragraph {
            content: t,
        }
    }
}
impl TryFrom<&mut VecDeque<Token>> for Paragraph {
    type Error = ParseError;
    fn try_from(t: &mut VecDeque<Token>) -> Result<Self> {
        let mut content = Vec::new();
        while let Ok(x) = Inline::try_from(&mut *t) {
            let is_line_break = x.is_line_break();
            content.push(x);
            if is_line_break {
                break;
            } 
        }
        Ok(Paragraph::from(content))
    }
}