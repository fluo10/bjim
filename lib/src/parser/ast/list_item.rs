use super::Inline;
use crate::parser::{
    errors::ParseError,
    token::{Token, TokenKind},
};


use std::collections::VecDeque;
use std::convert::{From, TryFrom};

use anyhow::anyhow;

type Result<T> = std::result::Result<T, ParseError>;

#[derive(Debug, PartialEq)]
pub struct ListItemPrefix {
    pub indent: Option<Token>,
    pub bullet: Token,
    pub space: Token,
}

impl From<&'_ mut VecDeque<Token>> for ListItemPrefix {
    fn from(tokens: &'_ mut VecDeque<Token>) -> ListItemPrefix {
        let indent: Option<Token>;
        if &tokens.front().unwrap().kind == &TokenKind::Indent {
            indent = tokens.pop_front();
        } else {
            indent = None;
        }
        ListItemPrefix {
            indent: indent,
            bullet: tokens.pop_front().unwrap(),
            space: tokens.pop_front().unwrap(),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Checkbox {
    pub open: Token,
    pub status: Token,
    pub close: Token,
    pub space: Token,
}

#[derive(Debug, PartialEq)]
pub struct ListItem {
    pub prefix: ListItemPrefix,
    pub checkbox: Option<Checkbox>,
    pub content: Vec<Inline>,
    pub children: Vec<ListItem>,
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

#[derive(Debug, PartialEq)]
pub struct PeekedListItemPrefix<'a>{
    pub indent: Option<&'a Token>,
    pub bullet: &'a Token,
    pub space: &'a Token,
}

impl<'a> TryFrom<(Option<&'a Token>, Option<&'a Token>, Option<&'a Token>)> for PeekedListItemPrefix<'a> {
    type Error = ParseError;
    fn try_from(tokens: (Option<&'a Token>, Option<&'a Token>, Option<&'a Token>)) -> Result<PeekedListItemPrefix<'a>> {
        match (tokens.0.map(|x| x.kind), tokens.1.map(|x| x.kind), tokens.2.map(|x| x.kind)) {
            (Some(TokenKind::Indent), Some(TokenKind::Bullet), Some(TokenKind::Space)) => {
                Ok(PeekedListItemPrefix{
                    indent: tokens.0,
                    bullet: tokens.1.unwrap(),
                    space: tokens.2.unwrap()
                })
            },
            (Some(TokenKind::Bullet), Some(TokenKind::Space), _) => {
                Ok(PeekedListItemPrefix{
                    indent: None,
                    bullet: tokens.0.unwrap(),
                    space: tokens.1.unwrap()
                })
            },
            (_, _, _) => {
                Err(anyhow!("Unexpected tokens").into())
            }
        }
    }
}