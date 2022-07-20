use crate::errors::ParseError;
use crate::token::*;

use std::collections::VecDeque;
use std::convert::{From, TryFrom};

type Result<T> = std::result::Result<T, ParseError>;

#[derive(Clone, Debug, PartialEq)]
pub enum InlineElement {
    Text(TextElement),
    LineBreak(LineBreakElement),
    //Bold,
    //Italic,
    //Link(),
    //HashTag(),
}

impl TryFrom<&mut VecDeque<LexedToken>> for InlineElement {
    type Error = ParseError;
    fn try_from(t: &mut VecDeque<LexedToken>) -> Result<Self> {
        use LexedToken::*;
        match &t.front(){
            Some(&LineBreak(_)) => {
                Ok(InlineElement::LineBreak(LineBreakElement::try_from(&mut *t).unwrap()))
            },
            Some(&LeftBracket(_)) => {
                todo!();
            },
            Some(&Hash(_)) => {
                todo!();
            },
            Some(_) => {
                Ok(InlineElement::Text(TextElement::try_from(&mut *t)?))
            },
            None => {
                Err(ParseError::TokenNotFound)
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct TextElement {
    pub content: Vec<LexedToken>,
}

impl TryFrom<&mut VecDeque<LexedToken>> for TextElement {
    type Error = ParseError;
    fn try_from(t: &mut VecDeque<LexedToken>) -> Result<Self> {
        todo!();
    }
}


#[derive(Clone, Debug, PartialEq)]
pub struct LineBreakElement {
    pub content: LineBreakToken,
}

impl From<LineBreakToken> for LineBreakElement {
    fn from(t: LineBreakToken) -> Self {
        Self {
            content: t,
        }
    }
}

impl TryFrom<&mut VecDeque<LexedToken>> for LineBreakElement {
    type Error = ParseError;
    fn try_from(t: &mut VecDeque<LexedToken>) -> Result<Self> {
        use LexedToken::*;
        match t.front() {
            Some(LineBreak(_)) => {
                if let LexedToken::LineBreak(x) = t.pop_front().unwrap() {
                    Ok(Self::from(x))
                } else {
                    panic!()
                }
            },
            _ => Err(ParseError::InvalidToken)

        }
    }
}