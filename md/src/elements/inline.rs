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

impl From<InlineElement> for Vec<ParsedToken> {
    fn from(i: InlineElement) -> Vec<ParsedToken> {
        match i {
            InlineElement::LineBreak(x) => x.into(),
            InlineElement::Link(x) => x.into(),
            InlineElement::Text(x) => x.into(),
        }
    }
}


#[derive(Clone, Debug, PartialEq)]
pub struct TextElement {
    pub content: TextToken,
}

impl From<Vec<LexedToken>> for TextElement {
    fn from(v: Vec<LexedToken>) -> Self {
        TextElement {
            content: v.into(),
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

impl From<TextElement> for Vec<ParsedToken> {
    fn from(e: TextElement) -> Vec<ParsedToken> {
        vec![e.content.into()]
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

impl From<LineBreakElement> for Vec<ParsedToken> {
    fn from(e: LineBreakElement) -> Vec<ParsedToken> {
        vec![e.content.into()]
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct LinkElement {
    open_bracket: LeftBracketToken,
    label: TextToken,
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

impl From<LinkElement> for Vec<ParsedToken> {
    fn from(e: LinkElement) -> Vec<ParsedToken> {
        vec![e.open_bracket.into(), e.label.into(), e.close_bracket.into()]
    }
}