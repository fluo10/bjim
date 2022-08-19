//mod blank_line;
//mod header;
//mod paragraph;
//mod list;

use crate::errors::ParseError;
use crate::token::*;
use crate::elements::*;

use std::fmt;

//pub use blank_line::BlankLine;
//pub use paragraph::Paragraph;
//pub use header::{Header, HeaderPrefix, PeekedHeaderPrefix};
//pub use list::{List, ListItem, ListItemPrefix, PeekedListItemPrefix};

use std::collections::VecDeque;
use std::convert::TryFrom;

type Result<T> = std::result::Result<T, ParseError>;


#[derive(Clone, Debug, PartialEq)]
pub struct BlankLineElement{
    pub indent: Option<LexedToken>,
    pub line_break: LineBreakToken,
}
impl From<LineBreakToken> for BlankLineElement {
    fn from(t: LineBreakToken) -> Self {
        Self{
            indent: None,
            line_break: t,
        }
    }
}

impl From<(SpaceToken, LineBreakToken)> for BlankLineElement {
    fn from(t: (SpaceToken, LineBreakToken)) -> Self {
        Self {
            indent: Some(t.0.into()),
            line_break: t.1,
        }
    }
}

impl TryFrom<&mut VecDeque<LexedToken>> for BlankLineElement {
    type Error = ParseError;
    fn try_from(t: &mut VecDeque<LexedToken>) -> Result<Self> {
        use LexedToken::*;
        match (t.get(0), t.get(1)) {
            (Some(LineBreak(_)), _) => {
                if let LineBreak(x) = t.pop_front().unwrap() {
                    Ok(Self::from(x))
                } else {
                    unreachable!()
                }
            },
            (Some(Space(_)), Some(LineBreak(_))) => {
                if let (Space(x), LineBreak(y)) = (t.pop_front().unwrap(), t.pop_front().unwrap()) {
                    Ok(Self::from((x, y)))
                } else {
                    unreachable!()
                }
            }
            _ => {
                Err(ParseError::InvalidToken)
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum BlockElement {
    BlankLine(BlankLineElement),
    List(ListElement),
    Paragraph(ParagraphElement),
}

impl From<BlankLineElement> for BlockElement {
    fn from (b: BlankLineElement) -> BlockElement {
        Self::BlankLine(b)
    }
}
impl From<ListElement> for BlockElement {
    fn from (l: ListElement) -> BlockElement {
        Self::List(l)
    }
}
impl From<ParagraphElement> for BlockElement {
    fn from (p: ParagraphElement) -> BlockElement {
        Self::Paragraph(p)
    }
}

impl TryFrom<&mut VecDeque<LexedToken>> for BlockElement {
    type Error = ParseError;
    fn try_from(t: &mut VecDeque<LexedToken>) -> Result<Self> {
        match (t.get(0), t.get(1)) {
            (Some(LexedToken::Hyphen(_)), Some(LexedToken::Space(_))) => {
                Ok(ListElement::try_from(&mut *t).unwrap().into())
            },
            (Some(LexedToken::LineBreak(_)),_) | 
            (Some(LexedToken::Space(_)), Some(LexedToken::LineBreak(_))) => {
                Ok(BlankLineElement::try_from(&mut *t).unwrap().into())
            },
            (Some(LexedToken::Hash(_)), Some(LexedToken::Space(_))) |
            (Some(LexedToken::Hash(_)), Some(LexedToken::Hash(_))) => {
                if crate::elements::section::peek_heading_level(& *t).is_some() {
                    Err(ParseError::InvalidToken)
                } else {
                    Ok(ParagraphElement::try_from(&mut *t).unwrap().into())
                }
            },
            (None, _) => {
                Err(ParseError::TokenNotFound)
            },
            _ => {
                Ok(ParagraphElement::try_from(&mut *t).unwrap().into())
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct ParagraphElement {
    pub content: Vec<InlineElement>,
}

impl From<Vec<InlineElement>> for ParagraphElement {
    fn from(t: Vec<InlineElement>) -> Self {
        ParagraphElement {
            content: t,
        }
    }
}

impl TryFrom<&mut VecDeque<LexedToken>> for ParagraphElement {
    type Error = ParseError;
    fn try_from(t: &mut VecDeque<LexedToken>) -> Result<Self> {
        let mut content = Vec::new();
        while let Ok(x) = InlineElement::try_from(&mut *t) {
            if let &InlineElement::LineBreak(_) = &x {
                content.push(x);
                break;
            } else {
                content.push(x);
            }
        }
        Ok(ParagraphElement::from(content))
    }
}
