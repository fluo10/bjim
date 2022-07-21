use crate::errors::ParseError;
use crate::token::*;
use crate::elements::*;

use std::collections::VecDeque;
use std::convert::TryFrom;

type Result<T> = std::result::Result<T, ParseError>;

#[derive(Clone, Debug, PartialEq)]
pub struct Section{
    pub header: Option<HeadingElement>,
    pub content: Vec<SectionContentElement>,
    pub children: Vec<Section>,
}

impl Section {
    fn new() -> Self {
        Section {
            header: None,
            content: Vec::new(),
            children: Vec::new(),
        }
    }
    fn heading_level(&self) -> u8 {
        self.header.as_ref().map_or(0, |v| v.heading_level())
    }
}

impl From<HeadingElement> for Section {
    fn from(h: HeadingElement) -> Self {
        Section{
            header: Some(h),
            content: Vec::new(),
            children: Vec::new(),
        }
    }
}

impl From<Vec<SectionContentElement>> for Section {
    fn from(v: Vec<SectionContentElement>) -> Self {
        Section{
            header: None,
            content: v,
            children: Vec::new(),
        }
    }
}

impl From<(HeadingElement, Vec<SectionContentElement>)> for Section {
    fn from(e:(HeadingElement, Vec<SectionContentElement>)) -> Self {
        Section{
            header: Some(e.0),
            content: e.1,
            children: Vec::new(),
        }
    }
}

impl From<(HeadingElement, Vec<SectionContentElement>, Vec<Section>)> for Section {
    fn from(e: (HeadingElement, Vec<SectionContentElement>, Vec<Section>)) -> Self {
        Section{
            header: Some(e.0),
            content: e.1,
            children: e.2,
        }
    }
}

impl From<(Vec<SectionContentElement>, Vec<Section>)> for Section {
    fn from(e: (Vec<SectionContentElement>, Vec<Section>)) -> Self {
        Section{
            header: None,
            content: e.0,
            children: e.1,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum SectionContentElement {
    BlankLine(BlankLineElement),
    List(ListElement),
    Paragraph(ParagraphElement),
}

impl From<BlankLineElement> for SectionContentElement {
    fn from (b: BlankLineElement) -> SectionContentElement {
        Self::BlankLine(b)
    }
}
impl From<ListElement> for SectionContentElement {
    fn from (l: ListElement) -> SectionContentElement {
        Self::List(l)
    }
}
impl From<ParagraphElement> for SectionContentElement {
    fn from (p: ParagraphElement) -> SectionContentElement {
        Self::Paragraph(p)
    }
}

impl TryFrom<&mut VecDeque<LexedToken>> for SectionContentElement {
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
                if peek_heading_level(& *t).is_some() {
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

impl TryFrom<&mut VecDeque<LexedToken>> for Section {
    type Error = ParseError;
    fn try_from(t: &mut VecDeque<LexedToken>) -> Result<Self> {
        let mut section = if let Ok(x) = HeadingElement::try_from(&mut *t) {
            Section::from(x)
        } else {
            Section::new()
        };
        while let Ok(x) = SectionContentElement::try_from(&mut *t) {
            section.content.push(x);
        }
        while let Some(x) = peek_heading_level(& *t) {
            if x > section.heading_level() {
                if let Ok(c) = Section::try_from(&mut *t) {
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
        if let LexedToken::Hash(x) = t.pop_front().unwrap() {
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

fn peek_heading_level(q: &VecDeque<LexedToken>) -> Option<u8> {
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
