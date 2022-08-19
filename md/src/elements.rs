mod block;
mod inline;
mod list;
mod section;
mod traits;


pub use block::*;
pub use inline::*;
pub use list::*;
pub use section::*;
pub use traits::*;

use crate::errors::ParseError;
use crate::token::*;

use std::collections::VecDeque;
use std::convert::{From, Into};
use std::fmt;

type Result<T> = std::result::Result<T, ParseError>;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct BodyElement {
    pub blocks: Vec<BlockElement>,
    pub sections: Vec<SectionElement>,
}
impl From<Vec<BlockElement>> for BodyElement {
    fn from(b: Vec<BlockElement>) -> Self {
        Self{
            blocks: b,
            sections: Vec::new(),
        }
    }
}

impl From<Vec<SectionElement>> for BodyElement {
    fn from(s: Vec<SectionElement>) -> Self {
        Self{
            blocks: Vec::new(),
            sections: s,
        }
    }
}

impl From<(Vec<BlockElement>, Vec<SectionElement>)> for BodyElement {
    fn from(e: (Vec<BlockElement>, Vec<SectionElement>) ) -> Self {
        Self{
            blocks: e.0,
            sections: e.1,
        }
    }
}

impl TryFrom<&mut VecDeque<LexedToken>> for BodyElement {
    type Error = ParseError;
    fn try_from(t: &mut VecDeque<LexedToken>) -> Result<BodyElement> {
        let mut body = Self::default();
        while let Ok(x) = BlockElement::try_from(&mut *t) {
            body.blocks.push(x);
        }
        while let Ok(c) = SectionElement::try_from(&mut *t) {
            body.sections.push(c);
        }
        Ok(body)
    }
}
