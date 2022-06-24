
use std::fmt;
use std::convert::From;
use super::{
    Header,
    Paragraph,
    BlankLine,
    Section,
    List,
};
use crate::parser::Token;

#[derive(Debug, PartialEq)]
pub enum Block {
    Header(Header),
    Paragraph(Paragraph),
    BlankLine(BlankLine),
    Section(Section),
    List(List),
}
/*
impl fmt::Display for Block {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, match self {
            Block::Header(x) => x.to_string(),
            Block::Paragraph(x) => x.to_string(),
            Block::BlankLine(x) => x.to_string(),
            Block::Section(x) => x.to_string(),
            Block::List(x) => x.to_string(),
        })
    }
}
*/

impl From<Header> for Block {
    fn from(h: Header) -> Self {
        Block::Header(h)
    }
}

impl From<Paragraph> for Block {
    fn from(p: Paragraph) -> Self {
        Block::Paragraph(p)
    }
}
impl From<BlankLine> for Block {
    fn from(b: BlankLine) -> Self {
        Block::BlankLine(b)
    }
}
impl From<Section> for Block {
    fn from(s: Section) -> Self {
        Block::Section(s)
    }
}
impl From<List> for Block {
    fn from(l: List) -> Self {
        Block::List(l)
    }
}