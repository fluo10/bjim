use crate::{
    errors::ParseError,
    token::{Token, TokenKind},
    elements::Inline,
};

use super::{ListItemPrefix, PeekedListItemPrefix, CheckBox};

use std::collections::VecDeque;
use std::convert::{From, TryFrom};

use anyhow::anyhow;

type Result<T> = std::result::Result<T, ParseError>;

#[derive(Debug, PartialEq)]
pub struct ListItem {
    pub prefix: ListItemPrefix,
    pub checkbox: Option<CheckBox>,
    pub content: Vec<Inline>,
    pub children: Vec<ListItem>,
}

impl TryFrom<&mut VecDeque<Token>> for ListItem {
    type Error = ParseError;
    fn try_from(t: &mut VecDeque<Token>) -> Result<Self>{

        let mut list_item: ListItem = ListItemPrefix::try_from(&mut *t)?.into();
        while let Ok(inline) = Inline::try_from(&mut *t) {
            if inline.is_line_break() {
                list_item.content.push(inline);
                break;
            } else {
                list_item.content.push(inline);
            }
        }
        while let Ok(prefix) = PeekedListItemPrefix::try_from(&* t) {
            if  prefix.indent.map_or(0, |x| x.literal.len()) > list_item.prefix.indent.as_ref().map_or(0, |x| x.literal.len()) {
                if let Ok(x) = ListItem::try_from(&mut *t) {
                    list_item.children.push(x);
                } else {
                    break;
                }
            } else {
                break;
            }
        }
        Ok(list_item)
    }
}

impl From<ListItemPrefix> for ListItem {
    fn from(p: ListItemPrefix) -> Self {
        ListItem{
            prefix: p,
            checkbox: None,
            content: Vec::new(),
            children: Vec::new(),
        }
    }
}
