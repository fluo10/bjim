pub mod check_box;
pub mod list_item;
pub mod list_item_prefix;

pub use check_box::*;
pub use list_item::*;
pub use list_item_prefix::*;

use crate::{
    errors::ParseError, 
    token::{Token, TokenKind},
};

use std::collections::VecDeque;
use std::convert::TryFrom;

type Result<T> = std::result::Result<T, ParseError>;

#[derive(Debug, PartialEq)]
pub struct List {
    pub content: Vec<ListItem>,
}

impl List {
    pub fn new() -> Self {
        List {
            content: Vec::new(),
        }
    }
}
impl TryFrom<&mut VecDeque<Token>> for List {
    type Error = ParseError;
    fn try_from(t: &mut VecDeque<Token>) -> Result<Self> {
        match PeekedListItemPrefix::try_from(&* t) {
            Ok(x) if x.indent.is_some() => {
                return Err(ParseError::InvalidToken);
            },
            Err(x) => {
                return Err(x);
            },
            _ => {}
        }
        let mut list = List::new();
        while let Ok(list_item) = ListItem::try_from(&mut *t) {
            list.content.push(list_item);
        }
        

        Ok(list)
        
    }
}