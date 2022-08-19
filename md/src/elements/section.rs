use crate::errors::ParseError;
use crate::token::*;
use crate::elements::*;

use std::collections::VecDeque;
use std::convert::TryFrom;

type Result<T> = std::result::Result<T, ParseError>;

#[derive(Clone, Debug, PartialEq)]
pub struct SectionElement{
    pub header: HeadingElement,
    pub content: Vec<BlockElement>,
    pub children: Vec<SectionElement>,
}

impl SectionElement {
    fn heading_level(&self) -> u8 {
        self.header.heading_level()
    }
}

impl From<HeadingElement> for SectionElement {
    fn from(h: HeadingElement) -> Self {
        SectionElement{
            header: h,
            content: Vec::new(),
            children: Vec::new(),
        }
    }
}

impl From<(HeadingElement, Vec<BlockElement>)> for SectionElement {
    fn from(e:(HeadingElement, Vec<BlockElement>)) -> Self {
        SectionElement{
            header: e.0,
            content: e.1,
            children: Vec::new(),
        }
    }
}

impl From<(HeadingElement, Vec<BlockElement>, Vec<SectionElement>)> for SectionElement {
    fn from(e: (HeadingElement, Vec<BlockElement>, Vec<SectionElement>)) -> Self {
        SectionElement{
            header: e.0,
            content: e.1,
            children: e.2,
        }
    }
}

impl TryFrom<&mut VecDeque<LexedToken>> for SectionElement {
    type Error = ParseError;
    fn try_from(t: &mut VecDeque<LexedToken>) -> Result<Self> {
        let mut section = SectionElement::from(HeadingElement::try_from(&mut *t)?);
        while let Ok(x) = BlockElement::try_from(&mut *t) {
            section.content.push(x);
        }
        while let Some(x) = peek_heading_level(& *t) {
            if x > section.heading_level() {
                if let Ok(c) = SectionElement::try_from(&mut *t) {
                    section.children.push(c);
                } else {
                    break;
                }
            } else {
                break;
            }
        }
        Ok(section)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct HeadingPrefix {
    pub prefix: HeadingPrefixToken,
    pub space: SpaceToken,
}

impl HeadingPrefix {
    pub fn heading_level(&self) -> u8 {
        self.prefix.len().try_into().unwrap()
    }
}

impl From<(HeadingPrefixToken, SpaceToken)> for HeadingPrefix {
    fn from(t: (HeadingPrefixToken, SpaceToken)) -> Self {
        Self {
            prefix : t.0,
            space : t.1,
        }
    }
}

impl TryFrom<&mut VecDeque<LexedToken>> for HeadingPrefix {
    type Error = ParseError;
    fn try_from(t:&mut VecDeque<LexedToken>) -> Result<Self> {
        if peek_heading_level(&*t).is_none(){
            return Err(ParseError::InvalidToken);
        }
    
        let mut prefix: HeadingPrefixToken;
        let space: SpaceToken;
        if let LexedToken::Hash(x) = t.pop_front().ok_or(ParseError::TokenNotFound)? {
            prefix = x.into();
        } else {
            unreachable!();
        }
        while let Some(x) = t.pop_front() {
            match x {
                LexedToken::Hash(y) => {
                    prefix +=  y;
                },
                LexedToken::Space(y) => {
                    space = y;
                    return Ok(HeadingPrefix::from((prefix, space)));
                    break;
                }
                _ => {
                    unreachable!();
                }
            }
        } 
        unreachable!();
    }
    
}

pub fn peek_heading_level(q: &VecDeque<LexedToken>) -> Option<u8> {
    let mut hl: u8 = 0;
    for t in q.iter() {
        match t {
            &LexedToken::Hash(_) => {
                hl += 1;
            },
            &LexedToken::Space(_) if hl > 0 => {
                break;
            },
            _ => {
                return None;
            }
        }
    }
    Some(hl)
}


#[derive(Clone, Debug, PartialEq)]
pub struct HeadingContent {
    content: Vec<InlineElement>,
}

impl TryFrom<&mut VecDeque<LexedToken>> for HeadingContent {
    type Error = ParseError;
    fn try_from(t: &mut VecDeque<LexedToken>) -> Result<Self>{
        todo!();
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct HeadingElement {
    pub prefix: HeadingPrefix,
    pub content: Vec<InlineElement>
}

impl HeadingElement {
    pub fn heading_level(&self) -> u8 {
        self.prefix.heading_level()
    }
}

impl From<(HeadingPrefixToken, SpaceToken, Vec<InlineElement>)> for HeadingElement {
    fn from(f: (HeadingPrefixToken, SpaceToken, Vec<InlineElement>)) -> Self {
        Self{
            prefix: (f.0, f.1).into(),
            content: f.2,
        }

    }
}
impl TryFrom<&mut VecDeque<LexedToken>> for HeadingElement {
    type Error = ParseError;
    fn try_from(t: &mut VecDeque<LexedToken>) -> Result<Self> {
        let prefix = HeadingPrefix::try_from(&mut *t)?;
        let mut inline: Vec<InlineElement> = Vec::new();
        while let Ok(x) = InlineElement::try_from(&mut *t) {
            if let &InlineElement::LineBreak(_) = &x {
                inline.push(x);
                break;
            } else {
                inline.push(x);
            }
        }
        Ok(HeadingElement{
            prefix: prefix,
            content: inline,
        })
    }
}
