mod blank_line;
mod header;
mod paragraph;
mod list;

use std::fmt;
pub use blank_line::BlankLine;
pub use paragraph::Paragraph;
pub use header::{Header, HeaderPrefix, PeekedHeaderPrefix};
pub use list::{List, ListItem, ListItemPrefix, PeekedListItemPrefix};

use crate::{
    errors::ParseError, 
    token::{Token, TokenKind},
};

use std::collections::VecDeque;
use std::convert::TryFrom;

type Result<T> = std::result::Result<T, ParseError>;

#[derive(Debug, PartialEq)]
pub enum Block {
    Paragraph(Paragraph),
    BlankLine(BlankLine),
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

impl From<List> for Block {
    fn from(l: List) -> Self {
        Block::List(l)
    }
}

impl TryFrom<&mut VecDeque<Token>> for Block {
    type Error = ParseError;
    fn try_from(t: &mut VecDeque<Token>)  -> Result<Block>{

        let block: Block = match (&t.front().ok_or(ParseError::TokenNotFound)?.kind, &t.get(1).map(|x| &x.kind)) {
            (&TokenKind::LineBreak, _) | (&TokenKind::Indent, Some(&TokenKind::LineBreak)) => {
                BlankLine::try_from(t).unwrap().into()
            },
            (&TokenKind::Bullet, Some(&TokenKind::Space)) => {
                List::try_from(&mut *t).map_or_else(|_| Paragraph::try_from(t).unwrap().into(), |x| x.into())
            },
            (_, _) => {
                Paragraph::try_from(t).unwrap().into()
            }
        };
        Ok(block)
    }
}