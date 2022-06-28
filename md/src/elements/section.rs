use crate::{
    errors::ParseError, 
    token::{Token, TokenKind},
    elements::block::{Header, HeaderPrefix, PeekedHeaderPrefix, Block},
};

use std::collections::VecDeque;
use std::convert::TryFrom;

type Result<T> = std::result::Result<T, ParseError>;

#[derive(Debug, PartialEq)]
pub struct Section{
    pub header: Option<Header>,
    pub content: Vec<Block>,
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

impl From<Header> for Section {
    fn from(h: Header) -> Self {
        Section{
            header: Some(h),
            content: Vec::new(),
            children: Vec::new(),
        }
    }
}

impl TryFrom<&mut VecDeque<Token>> for Section {
    type Error = ParseError;
    
    fn try_from(t: &mut VecDeque<Token>) -> Result<Self> {
        let mut section = if let Ok(x) = Header::try_from(&mut *t) {
            Section::from(x)
        } else {
            Section::new()
        };
        
        while PeekedHeaderPrefix::try_from(& *t).is_err() {
            if let Ok(b) = Block::try_from(&mut *t) {
                section.content.push(b);
            } else {
                break;
            }
        }
        while let Ok(x) = PeekedHeaderPrefix::try_from(& *t) {
            if x.heading_level() > section.heading_level() {
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
