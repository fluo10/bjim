
use std::fmt;
use super::{
    Header,
    Paragraph,
    BlankLine,
    Section,
    List,
};

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