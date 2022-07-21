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

type Result<T> = std::result::Result<T, ParseError>;

#[derive(Clone, Debug, PartialEq)]
pub struct Body {
    content: Section,
}

impl From<Section> for Body {
    fn from(s: Section) -> Self {
        Self{
            content: s,
        }
    }
}

impl TryFrom<&mut VecDeque<LexedToken>> for Body {
    type Error = ParseError;
    fn try_from(q: &mut VecDeque<LexedToken>) -> Result<Self> {
        Ok(Self::from(Section::try_from(&mut *q)?))
    }
}

