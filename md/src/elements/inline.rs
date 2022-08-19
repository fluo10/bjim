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
    Link(LinkElement),
    //HashTag(),
}

impl From<TextElement> for InlineElement {
    fn from(t: TextElement) -> Self {
        Self::Text(t)
    }
}
impl From<LineBreakElement> for InlineElement {
    fn from(t: LineBreakElement) -> Self {
        Self::LineBreak(t)
    }
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

impl From<Vec<LexedToken>> for TextElement {
    fn from(v: Vec<LexedToken>) -> Self {
        TextElement {
            content: v,
        }        
    }
}

impl TryFrom<&mut VecDeque<LexedToken>> for TextElement {
    type Error = ParseError;
    fn try_from(t: &mut VecDeque<LexedToken>) -> Result<Self> {
        if t.is_empty() {
            return Err(ParseError::TokenNotFound);
        }
        let mut v :Vec<LexedToken> = Vec::new(); 
        while let x = t.front().unwrap() {
            match x {
                &LexedToken::LineBreak(_) => {
                    break;
                },
                _ => {
                    v.push(t.pop_front().unwrap());
                }
            }
        }
        Ok(v.into())
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



#[derive(Clone, Debug, PartialEq)]
pub struct LinkElement {
    open_bracket: LeftBracketToken,
    label: Vec<LexedToken>,
    close_bracket: RightBracketToken,
    //open_parentheses: LeftParenthesis,
    //url: UrlToken,
    //close_parentheses: RightParenthesis,
}

impl From<()> for LinkElement {
    fn from(t: ()) -> Self {
        todo!()
    }
}

impl TryFrom<&mut VecDeque<LexedToken>> for LinkElement {
    type Error = ParseError;
    fn try_from(t: &mut VecDeque<LexedToken>) -> Result<Self> {
        todo!()
    }
}
